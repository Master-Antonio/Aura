use anyhow;
use ntapi::ntexapi::NtSetTimerResolution;
use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::command;
use tauri::ipc::InvokeError;
use thiserror::Error;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Error, Debug)]
pub enum OptimizationError {
    #[error("Failed to modify registry: {0}")]
    RegistryError(String),

    #[error("Failed to execute command: {0}")]
    CommandError(String),

    #[error("Operation not supported on this platform")]
    UnsupportedPlatform,

    #[error("Failed to set timer resolution: {0}")]
    TimerError(i32),
}

impl From<OptimizationError> for InvokeError {
    fn from(err: OptimizationError) -> Self {
        InvokeError::from_anyhow(anyhow::anyhow!(err))
    }
}

type Result<T> = std::result::Result<T, OptimizationError>;

// Cache per evitare chiamate ripetute al registro
lazy_static::lazy_static! {
    static ref REGISTRY_CACHE: Arc<Mutex<RegistryCache>> = Arc::new(Mutex::new(RegistryCache::new()));
}

struct RegistryCache {
    game_dvr_state: Option<bool>,
    irq_priority_state: Option<bool>,
}

impl RegistryCache {
    fn new() -> Self {
        Self {
            game_dvr_state: None,
            irq_priority_state: None,
        }
    }
}

#[command]
pub fn disable_game_dvr(enable: bool) -> Result<()> {
    let mut cache = REGISTRY_CACHE
        .lock()
        .map_err(|e| OptimizationError::RegistryError(e.to_string()))?;

    // Controlla la cache
    if cache.game_dvr_state == Some(enable) {
        return Ok(());
    }

    let value = if enable { "0" } else { "1" };
    let reg_path = r"HKEY_CURRENT_USER\System\GameConfigStore";

    modify_registry(reg_path, "GameDVR_Enabled", value)
        .map_err(|e| OptimizationError::RegistryError(e))?;

    // Aggiorna la cache
    cache.game_dvr_state = Some(enable);
    Ok(())
}

#[command]
pub fn optimize_interrupt_affinity(enable: bool) -> Result<()> {
    let mut cache = REGISTRY_CACHE
        .lock()
        .map_err(|e| OptimizationError::RegistryError(e.to_string()))?;

    if cache.irq_priority_state == Some(enable) {
        return Ok(());
    }

    let reg_path = r"HKEY_LOCAL_MACHINE\System\CurrentControlSet\Control\PriorityControl";

    if enable {
        modify_registry(reg_path, "IRQ8Priority", "1")
    } else {
        delete_registry_value(reg_path, "IRQ8Priority")
    }
        .map_err(|e| OptimizationError::RegistryError(e))?;

    cache.irq_priority_state = Some(enable);
    Ok(())
}

#[tauri::command]
pub fn optimize_time_resolution(enable: bool) -> Result<()> {
    #[cfg(target_os = "windows")]
    unsafe {
        let mut current_res: u32 = 0;
        let status = NtSetTimerResolution(10000, enable as u8, &mut current_res);

        if status >= 0 {
            Ok(())
        } else {
            Err(OptimizationError::TimerError(status))
        }
    }

    #[cfg(not(target_os = "windows"))]
    Err(OptimizationError::UnsupportedPlatform)
}

fn modify_registry(path: &str, key: &str, value: &str) -> std::result::Result<(), String> {
    #[cfg(target_os = "windows")]
    let output = Command::new("reg")
        .args(&["add", path, "/v", key, "/t", "REG_DWORD", "/d", value, "/f"])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("reg")
        .args(&["add", path, "/v", key, "/t", "REG_DWORD", "/d", value, "/f"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    } else {
        Ok(())
    }
}

fn delete_registry_value(path: &str, key: &str) -> std::result::Result<(), String> {
    #[cfg(target_os = "windows")]
    let output = Command::new("reg")
        .args(&["delete", path, "/v", key, "/f"])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("reg")
        .args(&["delete", path, "/v", key, "/f"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    } else {
        Ok(())
    }
}

// Low-level registry optimization functions
// These are used by the optimization service for actual system modifications

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_dvr_toggle() {
        let result = disable_game_dvr(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_timer_resolution() {
        let result = optimize_time_resolution(true);
        #[cfg(target_os = "windows")]
        assert!(result.is_ok());
        #[cfg(not(target_os = "windows"))]
        assert!(matches!(
            result,
            Err(OptimizationError::UnsupportedPlatform)
        ));
    }
}
