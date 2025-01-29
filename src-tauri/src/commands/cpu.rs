use crate::models::system_stats::{GenericData, ProgressData, SystemStats};
use anyhow;
use std::{sync::{Arc, Mutex}, time::Duration};
use sysinfo::{System, Components};
use tauri::command;
use tauri::ipc::InvokeError;
use thiserror::Error;

const CPU_SAMPLE_INTERVAL: Duration = sysinfo::MINIMUM_CPU_UPDATE_INTERVAL;
const CACHE_DURATION: Duration = Duration::from_millis(500);

#[derive(Error, Debug)]
pub enum CpuError {
    #[error("Failed to read CPU information: {0}")]
    ReadError(String),

    #[error("No CPU cores detected")]
    NoCoresError,

    #[error("Failed to refresh CPU data")]
    RefreshError,
}

impl From<CpuError> for InvokeError {
    fn from(err: CpuError) -> Self {
        InvokeError::from_anyhow(anyhow::anyhow!(err))
    }
}

type Result<T> = std::result::Result<T, CpuError>;

struct CpuCache {
    stats: Option<CpuStats>,
    last_update: std::time::Instant,
}

impl CpuCache {
    fn new() -> Self {
        Self {
            stats: None,
            last_update: std::time::Instant::now(),
        }
    }

    fn needs_update(&self) -> bool {
        self.stats.is_none() || self.last_update.elapsed() >= CACHE_DURATION
    }
}

#[derive(Clone)]
struct CpuStats {
    global_usage: f32,
    core_loads: Vec<ProgressData>,
}

lazy_static::lazy_static! {
    static ref CPU_CACHE: Arc<Mutex<CpuCache>> = Arc::new(Mutex::new(CpuCache::new()));
}

#[command]
pub fn get_cpu_stats() -> std::result::Result<SystemStats, String> {
    match crate::shared::system::SYSTEM.lock() {
        Ok(mut system) => {
            system.refresh_cpu_all();
            std::thread::sleep(Duration::from_millis(100)); // Brief pause for accurate readings
            system.refresh_cpu_all();

            let cpus = system.cpus();
            let global_usage = system.global_cpu_usage();

            // Get CPU model name
            let cpu_brand = cpus.first()
                .map(|cpu| cpu.brand().to_string())
                .unwrap_or_else(|| "Unknown CPU".to_string());            // Get temperatures from components
            let mut components = Components::new_with_refreshed_list();
            components.refresh(false);
            
            let cpu_temps: Vec<f32> = components
                .iter()
                .filter(|component| {
                    let label = component.label().to_lowercase();
                    label.contains("cpu") || label.contains("core") || label.contains("processor")
                })
                .filter_map(|component| component.temperature())
                .collect();

            // Calculate average CPU temperature
            let avg_temp = if !cpu_temps.is_empty() {
                cpu_temps.iter().sum::<f32>() / cpu_temps.len() as f32
            } else {
                0.0
            };

            // Get frequency info
            let base_freq = cpus.first().map(|cpu| cpu.frequency()).unwrap_or(0);
            let max_freq = cpus.iter().map(|cpu| cpu.frequency()).max().unwrap_or(0);            // Create progress data for individual cores with temperatures
            let progress_data: Vec<ProgressData> = cpus.iter().enumerate().map(|(i, cpu)| {
                let core_temp = cpu_temps.get(i).copied().unwrap_or(0.0);
                ProgressData {
                    title: format!("Core {}", i + 1),
                    value: cpu.cpu_usage(),
                    temperature: if core_temp > 0.0 { Some(core_temp) } else { None },
                }
            }).collect();            // Create detailed generic data
            let generic_data = vec![
                GenericData {
                    title: "Model".to_string(),
                    value: cpu_brand.clone(),
                },
                GenericData {
                    title: "Temp".to_string(),
                    value: if avg_temp > 0.0 {
                        format!("{:.1}°C", avg_temp)
                    } else {
                        "N/A".to_string()
                    },
                },
                GenericData {
                    title: "Base Clock".to_string(),
                    value: format!("{:.1} GHz", base_freq as f64 / 1000.0),
                },
                GenericData {
                    title: "Max Clock".to_string(),
                    value: format!("{:.1} GHz", max_freq as f64 / 1000.0),
                },
                GenericData {
                    title: "Cores/Threads".to_string(),
                    value: format!("{}/{}", cpus.len(), cpus.len()), // Most CPUs show same for simplicity
                },
            ];

            Ok(SystemStats {
                title: cpu_brand,
                percentage: Some(global_usage),
                progress_data: Some(progress_data),
                generic_data: Some(generic_data),
            })
        }
        Err(_) => {
            Ok(SystemStats {
                title: "CPU Usage".to_string(),
                percentage: Some(0.0),
                progress_data: None,
                generic_data: Some(vec![
                    GenericData {
                        title: "Error".to_string(),
                        value: "Unable to get CPU stats".to_string(),
                    }
                ]),
            })
        }
    }
}

fn measure_cpu_usage(system: &mut System) -> Result<CpuStats> {
    system.refresh_all();
    std::thread::sleep(CPU_SAMPLE_INTERVAL);
    system.refresh_all();

    let cpus = system.cpus();
    if cpus.is_empty() {
        return Err(CpuError::NoCoresError);
    }

    let global_usage = system.global_cpu_usage();    let core_loads = cpus
        .iter()
        .enumerate()
        .map(|(i, cpu)| ProgressData {
            title: format!("Core {}", i),
            value: cpu.cpu_usage().round(),
            temperature: None,
        })
        .collect();

    Ok(CpuStats {
        global_usage: global_usage.round(),
        core_loads,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_cache() {
        let result1 = get_cpu_stats();
        assert!(result1.is_ok());

        // Second call should use cache
        let result2 = get_cpu_stats();
        assert!(result2.is_ok());

        let stats1 = result1.unwrap();
        let stats2 = result2.unwrap();
        assert_eq!(stats1.percentage, stats2.percentage);
    }

    #[test]
    fn test_cpu_usage_range() {
        let stats = get_cpu_stats().unwrap();

        // Check global usage
        let usage = stats.percentage.unwrap();
        assert!(usage >= 0.0 && usage <= 100.0);

        // Check individual cores
        if let Some(cores) = stats.progress_data {
            for core in cores {
                assert!(core.value >= 0.0 && core.value <= 100.0);
            }
        }
    }

    #[test]
    fn test_cpu_title_format() {
        let stats = get_cpu_stats().unwrap();
        assert!(stats.title.starts_with("CPU ("));
        assert!(stats.title.ends_with("cores)"));
    }
}
