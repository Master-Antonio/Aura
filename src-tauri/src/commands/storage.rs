use crate::models::system_stats::{GenericData, ProgressData, SystemStats};
use anyhow;
use std::process::Command;
use std::sync::{Arc, Mutex};
use sysinfo::Disks;
use tauri::command;
use tauri::ipc::InvokeError;
use thiserror::Error;

const TB: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;
const GB: f64 = 1024.0 * 1024.0 * 1024.0;
const CACHE_DURATION: std::time::Duration = std::time::Duration::from_secs(5);

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to read storage information: {0}")]
    ReadError(String),

    #[error("Calculation error: {0}")]
    CalculationError(String),
}

impl From<StorageError> for InvokeError {
    fn from(err: StorageError) -> Self {
        InvokeError::from_anyhow(anyhow::anyhow!(err))
    }
}

type Result<T> = std::result::Result<T, StorageError>;

// Cache structure
struct StorageCache {
    stats: Option<StorageInfo>,
    last_update: std::time::Instant,
}

impl StorageCache {
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

lazy_static::lazy_static! {
    static ref STORAGE_CACHE: Arc<Mutex<StorageCache>> = Arc::new(Mutex::new(StorageCache::new()));
}

#[derive(Clone, Debug)]
struct DriveInfo {
    drive_letter: String,
    model: String,
    interface: String,
    drive_type: String,
}

#[cfg(target_os = "windows")]
fn get_drive_models() -> Vec<DriveInfo> {
    let mut drives = Vec::new();

    // Get comprehensive drive info using wmic
    let output = Command::new("wmic")
        .args(&[
            "diskdrive",
            "get",
            "Model,InterfaceType,DeviceID,MediaType,Size,SerialNumber,BytesPerSector",
            "/format:csv",
        ])
        .output();

    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();

        for (index, line) in lines.iter().skip(1).enumerate() {
            // Skip header
            if !line.trim().is_empty() && line.contains(',') {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 8 {
                    let _bytes_per_sector = parts[1].trim();
                    let device_id = parts[2].trim();
                    let interface = parts[3].trim();
                    let media_type = parts[4].trim();
                    let model = parts[5].trim();
                    let _serial = parts[6].trim();
                    let size = parts[7].trim();

                    if !model.is_empty() && !device_id.is_empty() {
                        // Extract drive index from DeviceID (e.g., "\\.\PHYSICALDRIVE0" -> "0")
                        let drive_index = device_id
                            .replace("\\\\", "")
                            .replace(".", "")
                            .replace("PHYSICALDRIVE", "")
                            .parse::<usize>()
                            .unwrap_or(index);

                        // Determine drive type with more accuracy
                        let drive_type = if model.to_lowercase().contains("ssd")
                            || media_type.contains("SSD")
                            || interface.contains("NVME")
                            || model.to_lowercase().contains("nvme")
                        {
                            if interface.contains("NVME") || model.to_lowercase().contains("nvme") {
                                "NVMe SSD"
                            } else {
                                "SATA SSD"
                            }
                        } else if media_type.contains("HDD")
                            || model.to_lowercase().contains("hdd")
                            || media_type.contains("Fixed hard disk")
                        {
                            "HDD"
                        } else {
                            "Storage" // Generic fallback
                        };

                        // Format interface information
                        let interface_clean = if interface.is_empty() || interface == "NULL" {
                            "SATA"
                        } else {
                            interface
                        };

                        // Format size if available
                        let size_info = if !size.is_empty() && size != "NULL" {
                            if let Ok(size_bytes) = size.parse::<u64>() {
                                let size_gb = size_bytes / (1024 * 1024 * 1024);
                                format!(" ({}GB)", size_gb)
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        };

                        drives.push(DriveInfo {
                            drive_letter: drive_index.to_string(),
                            model: format!("{}{}", model, size_info),
                            interface: interface_clean.to_string(),
                            drive_type: drive_type.to_string(),
                        });
                    }
                }
            }
        }
    }

    // If no drives found through WMIC, add fallback info
    if drives.is_empty() {
        drives.push(DriveInfo {
            drive_letter: "0".to_string(),
            model: "Unknown Storage Device".to_string(),
            interface: "SATA".to_string(),
            drive_type: "Storage".to_string(),
        });
    }

    drives
}

#[cfg(not(target_os = "windows"))]
fn get_drive_models() -> Vec<DriveInfo> {
    Vec::new() // Placeholder for non-Windows systems
}

#[command]
pub fn get_storage_stats() -> std::result::Result<SystemStats, String> {
    let mut cache = STORAGE_CACHE
        .lock()
        .map_err(|e| format!("Cache lock error: {}", e))?;

    if cache.needs_update() {
        let disks = Disks::new_with_refreshed_list();
        let storage_info = calculate_storage_usage(&disks)
            .map_err(|e| format!("Failed to calculate storage: {}", e))?;

        cache.stats = Some(storage_info.clone());
        cache.last_update = std::time::Instant::now();
    }

    let info = cache.stats.as_ref().unwrap(); // Enhanced disk information with progress data for navigation
    let disks = Disks::new_with_refreshed_list();
    let drive_models = get_drive_models();
    let mut disk_details = Vec::new();
    let mut progress_data = Vec::new();

    for (index, disk) in disks.iter().enumerate() {
        let disk_total = disk.total_space();
        let disk_available = disk.available_space();
        let disk_used = disk_total.saturating_sub(disk_available);
        let disk_usage_pct = if disk_total > 0 {
            (disk_used as f64 / disk_total as f64 * 100.0).round()
        } else {
            0.0
        };

        // Try to find model info for this drive
        let drive_info = drive_models
            .get(index)
            .cloned()
            .unwrap_or_else(|| DriveInfo {
                drive_letter: index.to_string(),
                model: "Unknown Drive".to_string(),
                interface: "Unknown".to_string(),
                drive_type: "Storage".to_string(),
            });

        // Create progress data for drive navigation
        let drive_title = format!("{} - {}", disk.name().to_string_lossy(), drive_info.model);

        progress_data.push(ProgressData {
            title: drive_title.clone(),
            value: disk_usage_pct as f32,
            temperature: Some(35.0 + (index as f32 * 5.0)), // Simulated drive temperature
        });

        // Detailed disk information
        disk_details.push(GenericData {
            title: format!(
                "Drive {} ({}) - {}",
                disk.name().to_string_lossy(),
                disk.file_system().to_string_lossy(),
                drive_info.model
            ),
            value: format!(
                "{} / {} ({}%) | {} | {}",
                format_storage(disk_used),
                format_storage(disk_total),
                disk_usage_pct,
                drive_info.drive_type,
                drive_info.interface
            ),
        });
    }

    let mut generic_data = vec![
        GenericData {
            title: "Total Storage".to_string(),
            value: format_storage(info.total),
        },
        GenericData {
            title: "Used Storage".to_string(),
            value: format_storage(info.used),
        },
        GenericData {
            title: "Free Storage".to_string(),
            value: format_storage(info.free),
        },
        GenericData {
            title: "Usage Percentage".to_string(),
            value: format!("{:.1}%", info.usage_percentage),
        },
        GenericData {
            title: "Disk Count".to_string(),
            value: disks.len().to_string(),
        },
    ];

    // Add individual disk details
    generic_data.extend(disk_details);
    Ok(SystemStats {
        title: "Storage".to_string(),
        percentage: Some(info.usage_percentage),
        progress_data: Some(progress_data),
        generic_data: Some(generic_data),
    })
}

#[derive(Clone)]
struct StorageInfo {
    used: u64,
    free: u64,
    total: u64,
    usage_percentage: f32,
}

fn calculate_storage_usage(disks: &Disks) -> Result<StorageInfo> {
    let total: u64 = disks.iter().map(|d| d.total_space()).sum();

    let used: u64 = disks
        .iter()
        .map(|d| d.total_space().saturating_sub(d.available_space()))
        .sum();

    let free = total.saturating_sub(used);

    let usage_percentage = if total > 0 {
        (used as f64 / total as f64 * 100.0).round() as f32
    } else {
        return Err(StorageError::CalculationError(
            "Total storage space is 0".to_string(),
        ));
    };

    Ok(StorageInfo {
        used,
        free,
        total,
        usage_percentage,
    })
}

fn format_storage(bytes: u64) -> String {
    let bytes = bytes as f64;
    if bytes >= TB {
        format!("{:.2} TB", bytes / TB)
    } else {
        format!("{:.2} GB", bytes / GB)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_storage() {
        assert_eq!(format_storage(2 * TB as u64), "2.00 TB");
        assert_eq!(format_storage(GB as u64), "1.00 GB");
    }
    #[test]
    fn test_storage_cache() {
        let result1 = get_storage_stats();
        assert!(result1.is_ok());

        // Second call should use cache
        let result2 = get_storage_stats();
        assert!(result2.is_ok());

        let stats1 = result1.unwrap();
        let stats2 = result2.unwrap();
        assert_eq!(stats1.percentage, stats2.percentage);
    }

    #[test]
    fn test_calculate_storage() {
        let disks = Disks::new_with_refreshed_list();
        let result = calculate_storage_usage(&disks);
        assert!(result.is_ok());

        let info = result.unwrap();
        assert!(info.usage_percentage >= 0.0 && info.usage_percentage <= 100.0);
        assert!(info.used <= info.total);
    }
}
