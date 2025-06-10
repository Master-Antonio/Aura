use crate::shared::system::{get_system, REFRESH_INTERVAL};
use crate::utils::bytes::format_bytes;
use anyhow::Result;
use std::sync::Arc;
use sysinfo::Pid;

pub fn name(pid: Arc<Pid>) -> Result<String> {
    let mut system = get_system()
        .lock()
        .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    system.refresh_all();

    Ok(system
        .process(*pid)
        .ok_or_else(|| anyhow::anyhow!("Process not found"))?
        .name()
        .to_string_lossy()
        .to_string())
}

pub fn parent_pid(pid: Arc<Pid>) -> Result<Option<i32>> {
    let mut system = get_system()
        .lock()
        .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    system.refresh_all();

    Ok(system
        .process(*pid)
        .and_then(|p| p.parent())
        .map(|ppid| ppid.as_u32() as i32))
}

pub fn session_id(pid: Arc<Pid>) -> Result<u32> {
    let mut system = get_system()
        .lock()
        .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    system.refresh_all();

    match system.process(*pid) {
        Some(process) => match process.session_id() {
            Some(session_id) => Ok(session_id.as_u32()),
            None => Ok(0),
        },
        None => Ok(0),
    }
}

pub fn user(pid: Arc<Pid>) -> Result<String> {
    let mut system = get_system()
        .lock()
        .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    system.refresh_all();

    Ok(system
        .process(*pid)
        .ok_or_else(|| anyhow::anyhow!("Process not found"))?
        .user_id()
        .map(|uid| uid.to_string())
        .unwrap_or_else(|| "Unknown".to_string()))
}

pub fn status(pid: Arc<Pid>) -> Result<String> {
    let mut system = get_system()
        .lock()
        .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    system.refresh_all();

    Ok(system
        .process(*pid)
        .ok_or_else(|| anyhow::anyhow!("Process not found"))?
        .status()
        .to_string())
}

pub fn cpu(pid: Arc<Pid>) -> Result<String> {
    let mut system = get_system()
        .lock()
        .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    system.refresh_all();
    std::thread::sleep(REFRESH_INTERVAL);
    system.refresh_all();

    let cpu_usage = system
        .process(*pid)
        .ok_or_else(|| anyhow::anyhow!("Process not found"))?
        .cpu_usage();

    Ok(format!("{:.1}%", cpu_usage))
}

pub fn memory(pid: Arc<Pid>) -> Result<String> {
    let mut system = get_system()
        .lock()
        .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    system.refresh_all();

    let memory = system
        .process(*pid)
        .ok_or_else(|| anyhow::anyhow!("Process not found"))?
        .memory();

    Ok(format_bytes(memory))
}

pub fn gpu(_pid: Arc<Pid>) -> Result<String> {
    // For now, return a placeholder since NVML is not available
    // TODO: Implement GPU usage detection using other methods
    Ok("N/A".to_string())
}

pub fn disk_io(pid: Arc<Pid>) -> Result<String> {
    let mut system = get_system()
        .lock()
        .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    system.refresh_all();

    let process = system
        .process(*pid)
        .ok_or_else(|| anyhow::anyhow!("Process not found"))?;

    let read_bytes = process.disk_usage().read_bytes;
    let write_bytes = process.disk_usage().written_bytes;

    Ok(format!(
        "R: {}/s, W: {}/s",
        format_bytes(read_bytes),
        format_bytes(write_bytes)
    ))
}

pub fn env_vars(pid: Arc<Pid>) -> Result<Vec<String>> {
    let mut system = get_system()
        .lock()
        .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    system.refresh_all();

    let env_vars = system
        .process(*pid)
        .ok_or_else(|| anyhow::anyhow!("Process not found"))?
        .environ()
        .iter()
        .map(|os_str| os_str.to_string_lossy().into_owned())
        .collect();

    Ok(env_vars)
}

pub fn children_processes(pid: Arc<Pid>) -> Result<Vec<i32>> {
    let mut system = get_system()
        .lock()
        .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
    system.refresh_all();

    Ok(system
        .processes()
        .values()
        .filter(|p| p.parent().map(|ppid| ppid == *pid).unwrap_or(false))
        .map(|p| p.pid().as_u32() as i32)
        .collect())
}
