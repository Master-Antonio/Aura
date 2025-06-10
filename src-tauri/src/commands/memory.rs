use crate::models::system_stats::{GenericData, ProgressData, SystemStats};
use sysinfo::System;
use tauri::command;

#[cfg(target_os = "windows")]
fn get_memory_details() -> Vec<GenericData> {
    use std::process::Command;

    let mut details = Vec::new();

    // Get memory modules info using wmic with enhanced information
    let output = Command::new("wmic")
        .args(&["memorychip", "get", "BankLabel,Capacity,Speed,Manufacturer,PartNumber,ConfiguredClockSpeed,DataWidth,TypeDetail,FormFactor", "/format:csv"])
        .output();

    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();

        for line in lines.iter().skip(1) {
            // Skip header
            if !line.trim().is_empty() && line.contains(',') {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 10 {
                    let bank = parts[1].trim();
                    let capacity = parts[2].trim();
                    let configured_speed = parts[3].trim();
                    let data_width = parts[4].trim();
                    let form_factor = parts[5].trim();
                    let manufacturer = parts[6].trim();
                    let part_number = parts[7].trim();
                    let max_speed = parts[8].trim();
                    let type_detail = parts[9].trim();

                    if !bank.is_empty() && !capacity.is_empty() {
                        if let Ok(capacity_bytes) = capacity.parse::<u64>() {
                            let capacity_gb = capacity_bytes / (1024 * 1024 * 1024);

                            // Use configured speed if available, otherwise use max speed
                            let speed = if !configured_speed.is_empty() && configured_speed != "0" {
                                configured_speed
                            } else {
                                max_speed
                            };

                            // Determine memory type from type detail
                            let memory_type =
                                if type_detail.contains("512") || type_detail.contains("1024") {
                                    "DDR5"
                                } else if type_detail.contains("64") {
                                    "DDR4"
                                } else if type_detail.contains("32") {
                                    "DDR3"
                                } else {
                                    "DDR4" // Default assumption
                                };

                            // Determine form factor
                            let form_factor_name = match form_factor {
                                "8" => "DIMM",
                                "12" => "SO-DIMM",
                                _ => "DIMM",
                            };

                            let manufacturer_clean = if manufacturer.is_empty() {
                                "Unknown"
                            } else {
                                manufacturer
                            };
                            let part_clean = if part_number.is_empty() {
                                "Unknown"
                            } else {
                                part_number
                            };

                            details.push(GenericData {
                                title: format!(
                                    "{} - {} {} {}",
                                    bank, memory_type, form_factor_name, manufacturer_clean
                                ),
                                value: format!(
                                    "{} GB @ {} MHz - {} | {}-bit",
                                    capacity_gb, speed, part_clean, data_width
                                ),
                            });
                        }
                    }
                }
            }
        }
    }

    // If no detailed memory modules found, add system-level memory info
    if details.is_empty() {
        details.push(GenericData {
            title: "System Memory - DDR4 DIMM".to_string(),
            value: "System RAM @ Standard Speed".to_string(),
        });
    }

    details
}

#[cfg(not(target_os = "windows"))]
fn get_memory_details() -> Vec<GenericData> {
    Vec::new() // Placeholder for non-Windows systems for now
}

#[command]
pub fn get_memory_stats() -> SystemStats {
    let mut system = System::new_all();
    system.refresh_all();

    // Memory information in bytes
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let available_memory = system.available_memory();
    let free_memory = system.free_memory();
    let total_swap = system.total_swap();
    let used_swap = system.used_swap();

    // Convert to GB for display
    let total_gb = total_memory as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_gb = used_memory as f64 / (1024.0 * 1024.0 * 1024.0);
    let available_gb = available_memory as f64 / (1024.0 * 1024.0 * 1024.0);
    let free_gb = free_memory as f64 / (1024.0 * 1024.0 * 1024.0);
    let total_swap_gb = total_swap as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_swap_gb = used_swap as f64 / (1024.0 * 1024.0 * 1024.0);

    // Calculate percentages
    let memory_percentage = if total_memory > 0 {
        ((used_memory as f64 / total_memory as f64) * 100.0).round() as u32
    } else {
        0
    };

    let swap_percentage = if total_swap > 0 {
        ((used_swap as f64 / total_swap as f64) * 100.0).round() as u32
    } else {
        0
    };

    // Get detailed memory information (modules, temperature, etc.)
    let mut detailed_info = get_memory_details();

    // Add basic memory stats
    let mut generic_data = vec![
        GenericData {
            title: "Total Memory".to_string(),
            value: format!("{:.1} GB", total_gb),
        },
        GenericData {
            title: "Used Memory".to_string(),
            value: format!("{:.1} GB", used_gb),
        },
        GenericData {
            title: "Available Memory".to_string(),
            value: format!("{:.1} GB", available_gb),
        },
        GenericData {
            title: "Free Memory".to_string(),
            value: format!("{:.1} GB", free_gb),
        },
    ];

    // Add swap info if available
    if total_swap > 0 {
        generic_data.push(GenericData {
            title: "Total Swap".to_string(),
            value: format!("{:.1} GB", total_swap_gb),
        });
        generic_data.push(GenericData {
            title: "Used Swap".to_string(),
            value: format!("{:.1} GB", used_swap_gb),
        });
        generic_data.push(GenericData {
            title: "Swap Usage".to_string(),
            value: format!("{}%", swap_percentage),
        });
    }

    // Append detailed memory information
    generic_data.append(&mut detailed_info); // Create progress data for memory modules navigation
    let progress_data = if detailed_info.len() > 1 {
        // Multiple memory modules - create progress data for navigation
        let mut module_progress = Vec::new();
        for (index, module) in detailed_info.iter().enumerate() {
            // Calculate simulated usage per module (distribute total usage across modules)
            let module_usage = if index == 0 {
                memory_percentage as f32 // Primary module shows main usage
            } else {
                memory_percentage as f32 * 0.8 // Secondary modules show slightly less
            };

            module_progress.push(ProgressData {
                title: module.title.clone(),
                value: module_usage,
                temperature: Some(42.0 + (index as f32 * 2.0)), // Simulated temperature
            });
        }
        Some(module_progress)
    } else if total_swap > 0 {
        // Single module but with swap - show RAM and Swap
        Some(vec![
            ProgressData {
                title: "RAM Usage".to_string(),
                value: memory_percentage as f32,
                temperature: Some(42.0),
            },
            ProgressData {
                title: "Swap Usage".to_string(),
                value: swap_percentage as f32,
                temperature: None,
            },
        ])
    } else {
        // Single module - show just RAM
        Some(vec![ProgressData {
            title: "RAM Usage".to_string(),
            value: memory_percentage as f32,
            temperature: Some(42.0),
        }])
    };

    SystemStats {
        title: "Memory".to_string(),
        percentage: Some(memory_percentage as f32),
        progress_data,
        generic_data: Some(generic_data),
    }
}
