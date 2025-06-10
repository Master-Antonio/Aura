use crate::models::system_stats::{GenericData, ProgressData, SystemStats};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::command;

const MONITOR_TIMEOUT: Duration = Duration::from_secs(10);
const CACHE_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_RETRIES: u32 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorHealth {
    pub cpu_healthy: bool,
    pub memory_healthy: bool,
    pub storage_healthy: bool,
    pub network_healthy: bool,
    pub gpu_healthy: bool,
    pub system_healthy: bool,
    pub last_health_check: u64,
    pub error_counts: HashMap<String, u32>,
}

#[derive(Clone)]
struct CachedStats {
    data: SystemStats,
    timestamp: Instant,
    is_fallback: bool,
}

struct ResilientMonitor {
    cached_stats: HashMap<String, CachedStats>,
    health_status: MonitorHealth,
    last_health_check: Instant,
    error_counts: HashMap<String, u32>,
}

impl ResilientMonitor {
    fn new() -> Self {
        Self {
            cached_stats: HashMap::new(),
            health_status: MonitorHealth {
                cpu_healthy: true,
                memory_healthy: true,
                storage_healthy: true,
                network_healthy: true,
                gpu_healthy: true,
                system_healthy: true,
                last_health_check: 0,
                error_counts: HashMap::new(),
            },
            last_health_check: Instant::now(),
            error_counts: HashMap::new(),
        }
    }

    fn should_use_cache(&self, stat_type: &str) -> bool {
        if let Some(cached) = self.cached_stats.get(stat_type) {
            cached.timestamp.elapsed() < CACHE_TIMEOUT
        } else {
            false
        }
    }

    fn get_cached_or_fallback(&self, stat_type: &str) -> Option<SystemStats> {
        self.cached_stats
            .get(stat_type)
            .map(|cached| cached.data.clone())
    }

    fn update_cache(&mut self, stat_type: String, stats: SystemStats) {
        self.cached_stats.insert(
            stat_type,
            CachedStats {
                data: stats,
                timestamp: Instant::now(),
                is_fallback: false,
            },
        );
    }

    fn record_error(&mut self, stat_type: &str) {
        let count = self.error_counts.entry(stat_type.to_string()).or_insert(0);
        *count += 1;

        // Update health status
        match stat_type {
            "cpu" => self.health_status.cpu_healthy = *count < MAX_RETRIES,
            "memory" => self.health_status.memory_healthy = *count < MAX_RETRIES,
            "storage" => self.health_status.storage_healthy = *count < MAX_RETRIES,
            "network" => self.health_status.network_healthy = *count < MAX_RETRIES,
            "gpu" => self.health_status.gpu_healthy = *count < MAX_RETRIES,
            "system" => self.health_status.system_healthy = *count < MAX_RETRIES,
            _ => {}
        }
    }

    fn reset_error_count(&mut self, stat_type: &str) {
        self.error_counts.insert(stat_type.to_string(), 0);

        // Reset health status
        match stat_type {
            "cpu" => self.health_status.cpu_healthy = true,
            "memory" => self.health_status.memory_healthy = true,
            "storage" => self.health_status.storage_healthy = true,
            "network" => self.health_status.network_healthy = true,
            "gpu" => self.health_status.gpu_healthy = true,
            "system" => self.health_status.system_healthy = true,
            _ => {}
        }
    }

    fn create_fallback_stats(&self, stat_type: &str) -> SystemStats {
        match stat_type {
            "cpu" => SystemStats {
                title: "CPU (Safe Mode)".to_string(),
                percentage: Some(0.0),
                progress_data: Some(vec![ProgressData {
                    title: "Core 0".to_string(),
                    value: 0.0,
                    temperature: None,
                }]),
                generic_data: Some(vec![GenericData {
                    title: "Status".to_string(),
                    value: "Monitoring temporarily unavailable".to_string(),
                }]),
            },
            "memory" => SystemStats {
                title: "Memory (Safe Mode)".to_string(),
                percentage: Some(0.0),
                progress_data: Some(vec![ProgressData {
                    title: "System Memory".to_string(),
                    value: 0.0,
                    temperature: None,
                }]),
                generic_data: Some(vec![GenericData {
                    title: "Status".to_string(),
                    value: "Memory monitoring temporarily unavailable".to_string(),
                }]),
            },
            "storage" => SystemStats {
                title: "Storage (Safe Mode)".to_string(),
                percentage: Some(0.0),
                progress_data: Some(vec![ProgressData {
                    title: "Primary Drive".to_string(),
                    value: 0.0,
                    temperature: None,
                }]),
                generic_data: Some(vec![GenericData {
                    title: "Status".to_string(),
                    value: "Storage monitoring temporarily unavailable".to_string(),
                }]),
            },
            "network" => SystemStats {
                title: "Network (Safe Mode)".to_string(),
                percentage: Some(0.0),
                progress_data: Some(vec![ProgressData {
                    title: "Primary Interface".to_string(),
                    value: 0.0,
                    temperature: None,
                }]),
                generic_data: Some(vec![GenericData {
                    title: "Status".to_string(),
                    value: "Network monitoring temporarily unavailable".to_string(),
                }]),
            },
            "system" => SystemStats {
                title: "System (Safe Mode)".to_string(),
                percentage: Some(0.0),
                progress_data: None,
                generic_data: Some(vec![GenericData {
                    title: "Status".to_string(),
                    value: "System monitoring temporarily unavailable".to_string(),
                }]),
            },
            _ => SystemStats {
                title: "Unknown (Safe Mode)".to_string(),
                percentage: Some(0.0),
                progress_data: None,
                generic_data: Some(vec![GenericData {
                    title: "Error".to_string(),
                    value: "Component temporarily unavailable".to_string(),
                }]),
            },
        }
    }
}

lazy_static::lazy_static! {
    static ref RESILIENT_MONITOR: Arc<Mutex<ResilientMonitor>> = Arc::new(Mutex::new(ResilientMonitor::new()));
}

#[command]
pub fn get_resilient_cpu_stats() -> Result<SystemStats, String> {
    resilient_stat_fetch("cpu", || super::cpu::get_cpu_stats())
}

#[command]
pub fn get_resilient_memory_stats() -> Result<SystemStats, String> {
    resilient_stat_fetch("memory", || Ok(super::memory::get_memory_stats()))
}

#[command]
pub fn get_resilient_storage_stats() -> Result<SystemStats, String> {
    resilient_stat_fetch("storage", || {
        super::storage::get_storage_stats().map_err(|e| e.to_string())
    })
}

#[command]
pub fn get_resilient_network_stats() -> Result<SystemStats, String> {
    resilient_stat_fetch("network", || super::network::get_network_stats())
}

#[command]
pub fn get_resilient_system_stats() -> Result<SystemStats, String> {
    resilient_stat_fetch("system", || super::system::get_system_stats())
}

#[command]
pub fn get_monitor_health() -> Result<MonitorHealth, String> {
    let mut monitor = RESILIENT_MONITOR
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    // Update health status
    monitor.health_status.last_health_check = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    monitor.health_status.error_counts = monitor.error_counts.clone();

    Ok(monitor.health_status.clone())
}

fn resilient_stat_fetch<F>(stat_type: &str, fetch_fn: F) -> Result<SystemStats, String>
where
    F: Fn() -> Result<SystemStats, String>,
{
    let mut monitor = RESILIENT_MONITOR
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    // Check if we should use cached data
    if monitor.should_use_cache(stat_type) {
        if let Some(cached_stats) = monitor.get_cached_or_fallback(stat_type) {
            return Ok(cached_stats);
        }
    }

    // Try to fetch fresh data with timeout protection
    let fetch_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| fetch_fn()));

    match fetch_result {
        Ok(Ok(stats)) => {
            // Success - update cache and reset error count
            monitor.update_cache(stat_type.to_string(), stats.clone());
            monitor.reset_error_count(stat_type);
            Ok(stats)
        }
        Ok(Err(_error)) => {
            // Controlled error - record and try fallback
            monitor.record_error(stat_type);

            if let Some(cached_stats) = monitor.get_cached_or_fallback(stat_type) {
                Ok(cached_stats) // Return cached data if available
            } else {
                let fallback_stats = monitor.create_fallback_stats(stat_type);
                monitor.update_cache(stat_type.to_string(), fallback_stats.clone());
                Ok(fallback_stats)
            }
        }
        Err(_panic) => {
            // Panic occurred - create safe fallback
            monitor.record_error(stat_type);
            let fallback_stats = monitor.create_fallback_stats(stat_type);
            monitor.update_cache(stat_type.to_string(), fallback_stats.clone());
            Ok(fallback_stats)
        }
    }
}

#[command]
pub fn reset_monitor_health() -> Result<(), String> {
    let mut monitor = RESILIENT_MONITOR
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    monitor.error_counts.clear();
    monitor.health_status = MonitorHealth {
        cpu_healthy: true,
        memory_healthy: true,
        storage_healthy: true,
        network_healthy: true,
        gpu_healthy: true,
        system_healthy: true,
        last_health_check: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        error_counts: HashMap::new(),
    };

    Ok(())
}
