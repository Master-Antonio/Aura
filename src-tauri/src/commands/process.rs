use crate::models::process_info::ProcessInfo;
use anyhow;
use std::sync::Arc;
use sysinfo::Pid;
use tauri::command;
use tauri::ipc::InvokeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Process not found: {0}")]
    NotFound(i32),

    #[error("Failed to read process information: {0}")]
    ReadError(String),

    #[error("Invalid process ID: {0}")]
    InvalidPid(i32),

    #[error("Failed to get process data: {0}")]
    DataError(String),
}

impl From<ProcessError> for InvokeError {
    fn from(err: ProcessError) -> Self {
        InvokeError::from_anyhow(anyhow::anyhow!(err))
    }
}

type Result<T> = std::result::Result<T, ProcessError>;

#[derive(Debug)]
struct ProcessData {
    name: String,
    parent_pid: Option<i32>,
    session_id: u32,
    user: String,
    status: String,
    cpu_usage: String,
    memory_usage: String,
    gpu_usage: Option<String>,
    disk_io: String,
    env_vars: Vec<String>,
    children: Vec<i32>,
}

#[command]
pub fn get_process_info(pid: i32) -> Result<ProcessInfo> {
    if pid <= 0 {
        return Err(ProcessError::InvalidPid(pid));
    }

    let pid = Pid::from(pid as usize);
    let pid = Arc::new(pid);

    let process_data = fetch_process_data(Arc::clone(&pid))?;

    Ok(ProcessInfo {
        pid: pid.as_u32() as i32,
        name: process_data.name,
        parent_pid: process_data.parent_pid,
        session_id: process_data.session_id as i32,
        user: process_data.user,
        status: process_data.status,
        cpu: process_data.cpu_usage,
        memory: process_data.memory_usage,
        gpu_usage: process_data.gpu_usage.unwrap_or_else(|| "N/A".to_string()),
        network: String::new(), // TODO: Implementare monitoraggio rete per processo
        disk_io: process_data.disk_io,
        env_vars: process_data.env_vars,
        children_processes: process_data.children,
    })
}

fn fetch_process_data(pid: Arc<Pid>) -> Result<ProcessData> {
    let name = crate::services::process_service::name(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;
    let parent_pid = crate::services::process_service::parent_pid(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;
    let session_id = crate::services::process_service::session_id(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;
    let user = crate::services::process_service::user(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;
    let status = crate::services::process_service::status(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;
    let cpu_usage = crate::services::process_service::cpu(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;
    let memory_usage = crate::services::process_service::memory(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;
    let gpu_usage = crate::services::process_service::gpu(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;
    let disk_io = crate::services::process_service::disk_io(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;
    let env_vars = crate::services::process_service::env_vars(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;
    let children = crate::services::process_service::children_processes(Arc::clone(&pid))
        .map_err(|e| ProcessError::ReadError(e.to_string()))?;

    Ok(ProcessData {
        name,
        parent_pid,
        session_id,
        user,
        status,
        cpu_usage,
        memory_usage,
        gpu_usage: Some(gpu_usage),
        disk_io,
        env_vars,
        children,
    })
}

#[command]
pub fn open_file_location(path: String) -> Result<()> {
    if path.is_empty() || path == "N/A" {
        return Err(ProcessError::DataError("Invalid file path".to_string()));
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        // Extract directory from file path
        let path_obj = std::path::Path::new(&path);
        let dir = if path_obj.is_file() {
            path_obj.parent().unwrap_or(path_obj)
        } else {
            path_obj
        };

        let result = Command::new("explorer").arg("/select,").arg(&path).spawn();
        match result {
            Ok(_) => Ok(()),
            Err(_e) => {
                // Fallback: just open the directory
                let _ = Command::new("explorer").arg(dir).spawn();
                Ok(())
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let result = Command::new("open").arg("-R").arg(&path).spawn();

        match result {
            Ok(_) => Ok(()),
            Err(_) => {
                // Fallback: open directory
                let path_obj = std::path::Path::new(&path);
                let dir = if path_obj.is_file() {
                    path_obj.parent().unwrap_or(path_obj)
                } else {
                    path_obj
                };
                let _ = Command::new("open").arg(dir).spawn();
                Ok(())
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        let path_obj = std::path::Path::new(&path);
        let dir = if path_obj.is_file() {
            path_obj.parent().unwrap_or(path_obj)
        } else {
            path_obj
        };

        // Try various file managers
        let managers = ["nautilus", "dolphin", "thunar", "pcmanfm", "nemo"];

        for manager in &managers {
            if let Ok(_) = Command::new(manager).arg(dir).spawn() {
                return Ok(());
            }
        }

        // Fallback to xdg-open
        let _ = Command::new("xdg-open").arg(dir).spawn();
        Ok(())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err(ProcessError::DataError(
            "Platform not supported".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_pid() {
        let result = get_process_info(0);
        assert!(matches!(result, Err(ProcessError::InvalidPid(0))));
    }

    #[test]
    fn test_current_process() {
        let pid = std::process::id() as i32;
        let result = get_process_info(pid);
        assert!(result.is_ok(), "Should get current process info");

        let info = result.unwrap();
        assert_eq!(info.pid, pid);
        assert!(!info.name.is_empty());
    }

    #[test]
    fn test_process_data_format() {
        let pid = std::process::id() as i32;
        let info = get_process_info(pid).unwrap();

        assert!(!info.cpu.is_empty());
        assert!(!info.memory.is_empty());
        assert!(!info.status.is_empty());
        assert!(info.children_processes.is_empty() || info.children_processes.len() > 0);
    }
}
