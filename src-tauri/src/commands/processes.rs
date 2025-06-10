use crate::models::process_info::{ProcessFilter, ProcessStatus};
use crate::models::system_stats::{GenericData, SystemStats};
use crate::services::process_control;
use crate::shared::system::get_system;
use crate::utils::{bytes::format_bytes, time::format_run_time};
use anyhow;
use regex;
use serde::{Deserialize, Serialize};
use sysinfo;
use tauri::command;
use tauri::ipc::InvokeError;
use thiserror::Error;

// Frontend-compatible filter structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontendProcessFilter {
    pub search_query: Option<String>,
    pub status: Option<String>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub min_cpu: Option<f32>,
    pub min_memory: Option<u64>, // in bytes
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

// Frontend-compatible process data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontendProcessData {
    pub name: String,
    pub pid: u32,
    pub cpu_usage: f64,
    pub exe_path: String,
    pub affinity_set: bool,
    pub ram_usage: u64, // in MB
    pub run_time: String,
    pub status: String,
    pub disk_usage: FrontendDiskUsage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontendDiskUsage {
    pub read: String,
    pub write: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResponse {
    pub processes: Vec<FrontendProcessData>,
    pub total_count: usize,
}

#[derive(Error, Debug)]
pub enum ProcessesError {
    #[error("Failed to read processes: {0}")]
    ReadError(String),

    #[error("Failed to filter processes: {0}")]
    FilterError(String),

    #[error("Process control error: {0}")]
    ControlError(#[from] process_control::ProcessControlError),
}

impl From<ProcessesError> for InvokeError {
    fn from(err: ProcessesError) -> Self {
        InvokeError::from_anyhow(anyhow::anyhow!(err))
    }
}

type Result<T> = std::result::Result<T, ProcessesError>;

#[derive(Debug)]
struct ProcessEntry {
    pid: i32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
    status: ProcessStatus,
    run_time: u64,
}

#[command]
pub fn get_processes(filter: ProcessFilter) -> Result<Vec<SystemStats>> {
    let mut system = get_system()
        .lock()
        .map_err(|e| ProcessesError::ReadError(e.to_string()))?;
    system.refresh_all();

    let processes = system.processes();
    let mut process_list = Vec::new();

    for (pid, process) in processes {
        let status = ProcessStatus::from(process.status().to_string().as_str());

        // Applica i filtri
        if let Some(name_filter) = &filter.name {
            if !process.name().to_string_lossy().contains(name_filter) {
                continue;
            }
        }

        if let Some(status_filter) = &filter.status {
            if status != *status_filter {
                continue;
            }
        }

        if let Some(min_cpu) = filter.min_cpu {
            if process.cpu_usage() < min_cpu {
                continue;
            }
        }

        let memory = process.memory();
        if let Some(min_memory) = filter.min_memory {
            if memory < min_memory {
                continue;
            }
        }

        let entry = ProcessEntry {
            pid: pid.as_u32() as i32,
            name: process.name().to_string_lossy().into_owned(),
            cpu_usage: process.cpu_usage(),
            memory_usage: memory,
            status,
            run_time: process.run_time(),
        };

        process_list.push(format_process_entry(&entry));
    }

    Ok(process_list)
}

fn format_process_entry(process: &ProcessEntry) -> SystemStats {
    let generic_data = vec![
        GenericData {
            title: "PID".to_string(),
            value: process.pid.to_string(),
        },
        GenericData {
            title: "CPU Usage".to_string(),
            value: format!("{:.1}%", process.cpu_usage),
        },
        GenericData {
            title: "Memory".to_string(),
            value: format_bytes(process.memory_usage),
        },
        GenericData {
            title: "Status".to_string(),
            value: format!("{:?}", process.status),
        },
        GenericData {
            title: "Run Time".to_string(),
            value: format_run_time(process.run_time),
        },
    ];

    SystemStats {
        title: process.name.clone(),
        percentage: Some(process.cpu_usage),
        progress_data: None,
        generic_data: Some(generic_data),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_filtering() {
        let filter = ProcessFilter::new()
            .with_min_cpu(0.0)
            .with_status(ProcessStatus::Running);

        let result = get_processes(filter);
        assert!(result.is_ok());

        let processes = result.unwrap();
        assert!(!processes.is_empty());

        // Verifica che tutti i processi abbiano i dati richiesti
        for process in processes {
            assert!(!process.title.is_empty());
            assert!(process.percentage.is_some());
            assert!(process.generic_data.is_some());
        }
    }
}

#[command]
pub fn boost_process_for_gaming(pid: u32) -> Result<()> {
    process_control::boost_process_for_gaming(pid).map_err(ProcessesError::ControlError)
}

#[command]
pub fn set_process_affinity(pid: u32, cores: Vec<u32>) -> Result<()> {
    process_control::set_process_affinity_cores(pid, cores).map_err(ProcessesError::ControlError)
}

#[command]
pub fn get_process_affinity(pid: u32) -> Result<Vec<u32>> {
    process_control::get_process_affinity(pid).map_err(ProcessesError::ControlError)
}

#[command]
pub fn get_cpu_core_count() -> Result<u32> {
    let system = get_system()
        .lock()
        .map_err(|e| ProcessesError::ReadError(e.to_string()))?;
    Ok(system.cpus().len() as u32)
}

#[command]
pub fn kill_process(pid: u32) -> Result<()> {
    let result = process_control::kill_process(pid).map_err(ProcessesError::ControlError);

    // Forza refresh del sistema per rimuovere processi terminati
    if result.is_ok() {
        if let Ok(mut system) = get_system().lock() {
            system.refresh_all();
        }
    }

    result
}

#[command]
pub fn suspend_process(pid: u32) -> Result<()> {
    process_control::suspend_process(pid).map_err(ProcessesError::ControlError)
}

#[command]
pub fn resume_process(pid: u32) -> Result<()> {
    process_control::resume_process(pid).map_err(ProcessesError::ControlError)
}

#[command]
pub async fn get_running_processes(filter: FrontendProcessFilter) -> Result<ProcessResponse> {
    #[cfg(target_os = "windows")]
    {
        // Use optimized Windows native API for much better performance
        match get_running_processes_native(filter.clone()).await {
            Ok(response) => return Ok(response),
            Err(_e) => {
                /*eprintln!(
                    "Failed to get processes with native API, falling back to sysinfo: {}",
                    e
                );*/
                // Fall through to sysinfo implementation
            }
        }
    }

    // Fallback implementation using sysinfo (for non-Windows or when native fails)
    get_running_processes_fallback(filter).await
}

#[cfg(target_os = "windows")]
async fn get_running_processes_native(filter: FrontendProcessFilter) -> Result<ProcessResponse> {
    let mut filtered_processes = Vec::new();

    // Pre-compile regex if needed for search
    let search_regex = if let Some(ref search_query) = filter.search_query {
        if !search_query.is_empty() {
            Some(regex::Regex::new(&regex::escape(&search_query.to_lowercase())).ok())
        } else {
            None
        }
    } else {
        None
    };

    // Use optimized Windows native API for much better performance
    let processes_info = process_control::get_all_processes_info()
        .map_err(|e| ProcessesError::ReadError(format!("Native API failed: {}", e)))?;

    // Process filtering with native data
    for process_info in processes_info.iter() {
        let process_name = &process_info.name;

        // Apply filters with early return for performance
        if let Some(ref search_query) = filter.search_query {
            if !search_query.is_empty() {
                if let Some(ref regex) = search_regex.as_ref().and_then(|r| r.as_ref()) {
                    if !regex.is_match(&process_name.to_lowercase()) {
                        continue;
                    }
                } else if !process_name
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                {
                    continue;
                }
            }
        }

        // Status filter
        if let Some(ref status_filter) = filter.status {
            if !status_filter.is_empty() {
                let normalized_status = if process_info.is_suspended {
                    "suspended"
                } else {
                    "runnable" // For simplicity, treat all non-suspended as runnable
                };
                if normalized_status != status_filter.as_str() {
                    continue;
                }
            }
        }

        // CPU filter - Now we have real-time CPU data from native API
        if let Some(min_cpu) = filter.min_cpu {
            if process_info.cpu_usage_percent < min_cpu as f64 {
                continue;
            }
        }
        // Memory filter (working set size)
        if let Some(min_memory) = filter.min_memory {
            if process_info.memory_working_set < min_memory {
                continue;
            }
        }

        // Calculate runtime in a more readable format
        let run_time = if process_info.create_time > 0 {
            // Convert Windows FILETIME to duration
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            let create_time_secs = process_info.create_time / 10_000_000 - 11644473600; // Convert FILETIME to Unix timestamp
            let runtime_secs = (current_time - create_time_secs).max(0) as u64;
            format_run_time(runtime_secs)
        } else {
            "Unknown".to_string()
        };

        let status = if process_info.is_suspended {
            "suspended"
        } else {
            "runnable"
        };
        let entry = FrontendProcessData {
            pid: process_info.pid,
            name: process_name.clone(),
            cpu_usage: process_info.cpu_usage_percent,
            exe_path: process_info.exe_path.clone(),
            affinity_set: false,
            ram_usage: process_info.memory_working_set / (1024 * 1024), // Convert to MB
            run_time,
            status: status.to_string(),
            disk_usage: FrontendDiskUsage {
                read: format_bytes(process_info.io_read_bytes),
                write: format_bytes(process_info.io_write_bytes),
            },
        };

        filtered_processes.push(entry);
    }

    // Sort processes
    sort_processes(&mut filtered_processes, &filter);

    let total_count = filtered_processes.len();

    // Apply pagination
    let paginated_processes = paginate_processes(filtered_processes, &filter);

    Ok(ProcessResponse {
        processes: paginated_processes,
        total_count,
    })
}

async fn get_running_processes_fallback(filter: FrontendProcessFilter) -> Result<ProcessResponse> {
    // Create a fresh system instance instead of using the shared one to avoid stale cache
    let mut system = sysinfo::System::new_all();

    // Force refresh to get absolutely current data
    system.refresh_all();

    // Small delay to ensure system has fresh data
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    system.refresh_all();

    let processes = system.processes();
    let mut filtered_processes = Vec::new();

    // Pre-compile regex if needed for search
    let search_regex = if let Some(ref search_query) = filter.search_query {
        if !search_query.is_empty() {
            Some(regex::Regex::new(&regex::escape(&search_query.to_lowercase())).ok())
        } else {
            None
        }
    } else {
        None
    };

    // Process filtering without any cache
    for (pid, process) in processes.iter() {
        let pid_u32 = pid.as_u32();
        let status = process.status().to_string();
        let cpu_usage = process.cpu_usage();
        let memory_usage = process.memory();
        let process_name = process.name().to_string_lossy().into_owned();

        // Always check suspension status for all processes to ensure accuracy
        // Use unwrap_or(false) to handle cases where suspension check fails (e.g., access denied)
        let is_suspended = process_control::is_process_suspended(pid_u32).unwrap_or(false);

        // Apply filters with early return for performance
        if let Some(ref search_query) = filter.search_query {
            if !search_query.is_empty() {
                if let Some(ref regex) = search_regex.as_ref().and_then(|r| r.as_ref()) {
                    if !regex.is_match(&process_name.to_lowercase()) {
                        continue;
                    }
                } else if !process_name
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                {
                    continue;
                }
            }
        }

        // Status filter
        if let Some(ref status_filter) = filter.status {
            if !status_filter.is_empty() {
                let normalized_status = normalize_process_status(&status, Some(is_suspended));
                if normalized_status != status_filter.as_str() {
                    continue;
                }
            }
        }

        // CPU filter
        if let Some(min_cpu) = filter.min_cpu {
            if cpu_usage < min_cpu {
                continue;
            }
        }

        // Memory filter
        if let Some(min_memory) = filter.min_memory {
            if memory_usage < min_memory {
                continue;
            }
        }

        // Always use the suspension status for display
        let final_status = normalize_process_status(&status, Some(is_suspended));

        let entry = FrontendProcessData {
            pid: pid_u32,
            name: process_name,
            cpu_usage: cpu_usage as f64,
            exe_path: process
                .exe()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|| "N/A".to_string()),
            affinity_set: false, // TODO: Implement affinity checking
            ram_usage: memory_usage / (1024 * 1024), // Convert to MB
            run_time: format_run_time(process.run_time()),
            status: final_status.to_string(),
            disk_usage: FrontendDiskUsage {
                read: "0".to_string(),
                write: "0".to_string(),
            },
        };

        filtered_processes.push(entry);
    }

    // Sort processes
    sort_processes(&mut filtered_processes, &filter);

    let total_count = filtered_processes.len();

    // Apply pagination
    let paginated_processes = paginate_processes(filtered_processes, &filter);

    Ok(ProcessResponse {
        processes: paginated_processes,
        total_count,
    })
}

// Helper functions
fn normalize_process_status(status: &str, is_suspended: Option<bool>) -> &'static str {
    // Check if process is suspended first
    if let Some(true) = is_suspended {
        return "suspended";
    }

    match status.to_lowercase().as_str() {
        "running" | "runnable" => "runnable",
        "sleeping" | "sleep" | "idle" => "sleeping",
        "stopped" | "zombie" | "dead" => "stopped",
        _ => "unknown",
    }
}

fn sort_processes(processes: &mut Vec<FrontendProcessData>, filter: &FrontendProcessFilter) {
    if let (Some(sort_by), Some(sort_order)) = (&filter.sort_by, &filter.sort_order) {
        let ascending = sort_order == "asc";

        match sort_by.as_str() {
            "name" => {
                if ascending {
                    processes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                } else {
                    processes.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
                }
            }
            "cpu" => {
                if ascending {
                    processes.sort_by(|a, b| {
                        a.cpu_usage
                            .partial_cmp(&b.cpu_usage)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    processes.sort_by(|a, b| {
                        b.cpu_usage
                            .partial_cmp(&a.cpu_usage)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
            }
            "memory" => {
                if ascending {
                    processes.sort_by(|a, b| a.ram_usage.cmp(&b.ram_usage));
                } else {
                    processes.sort_by(|a, b| b.ram_usage.cmp(&a.ram_usage));
                }
            }
            "pid" => {
                if ascending {
                    processes.sort_by(|a, b| a.pid.cmp(&b.pid));
                } else {
                    processes.sort_by(|a, b| b.pid.cmp(&a.pid));
                }
            }
            _ => {}
        }
    }
}

fn paginate_processes(
    processes: Vec<FrontendProcessData>,
    filter: &FrontendProcessFilter,
) -> Vec<FrontendProcessData> {
    let page = filter.page.unwrap_or(0); // 0-based page indexing to match frontend
    let page_size = filter.per_page.unwrap_or(50).min(1000); // Max 1000 items per page

    let start_index = (page * page_size) as usize;
    let end_index = (start_index + page_size as usize).min(processes.len());

    if start_index >= processes.len() {
        return Vec::new();
    }

    processes[start_index..end_index].to_vec()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessBasicInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage_percent: f64,
    pub memory_working_set: u64, // in MB
    pub is_suspended: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessDetailedInfo {
    pub pid: u32,
    pub parent_pid: u32,
    pub name: String,
    pub exe_path: String,
    pub cpu_usage_percent: f64,
    pub memory_working_set: u64, // in MB
    pub memory_private: u64,     // in MB
    pub memory_virtual: u64,     // in MB
    pub memory_pagefile: u64,    // in MB
    pub handle_count: u32,
    pub thread_count: u32,
    pub is_suspended: bool,
    pub session_id: u32,
    pub io_read_bytes: String,
    pub io_write_bytes: String,
    pub io_read_operations: u64,
    pub io_write_operations: u64,
    pub run_time: String,
    pub children: Vec<ProcessBasicInfo>,
}

#[command]
pub async fn get_detailed_process_info(pid: u32) -> Result<ProcessDetailedInfo> {
    let process_info =
        process_control::get_process_detailed_info(pid).map_err(ProcessesError::ControlError)?;

    // Get child processes
    let children =
        process_control::get_child_processes(pid).map_err(ProcessesError::ControlError)?;

    let detailed_info = ProcessDetailedInfo {
        pid: process_info.pid,
        parent_pid: process_info.parent_pid,
        name: process_info.name,
        exe_path: process_info.exe_path,
        cpu_usage_percent: process_info.cpu_usage_percent,
        memory_working_set: process_info.memory_working_set / (1024 * 1024), // Convert to MB
        memory_private: process_info.memory_private / (1024 * 1024),         // Convert to MB
        memory_virtual: process_info.memory_virtual / (1024 * 1024),         // Convert to MB
        memory_pagefile: process_info.memory_pagefile / (1024 * 1024),       // Convert to MB
        handle_count: process_info.handle_count,
        thread_count: process_info.thread_count,
        is_suspended: process_info.is_suspended,
        session_id: process_info.session_id,
        io_read_bytes: format_bytes(process_info.io_read_bytes),
        io_write_bytes: format_bytes(process_info.io_write_bytes),
        io_read_operations: process_info.io_read_operations,
        io_write_operations: process_info.io_write_operations,
        run_time: if process_info.create_time > 0 {
            // Convert Windows FILETIME to duration
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            let create_time_secs = process_info.create_time / 10_000_000 - 11644473600; // Convert FILETIME to Unix timestamp
            let runtime_secs = (current_time - create_time_secs).max(0) as u64;
            format_run_time(runtime_secs)
        } else {
            "Unknown".to_string()
        },
        children: children
            .into_iter()
            .map(|child| ProcessBasicInfo {
                pid: child.pid,
                name: child.name,
                cpu_usage_percent: child.cpu_usage_percent,
                memory_working_set: child.memory_working_set / (1024 * 1024), // Convert to MB
                is_suspended: child.is_suspended,
            })
            .collect(),
    };

    Ok(detailed_info)
}
