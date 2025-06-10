use crate::models::optimization::{OptimizationCategory, OptimizationResult};
use crate::services::optimization_service::OptimizationService;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::command;

lazy_static::lazy_static! {
    static ref OPTIMIZATION_SERVICE: Arc<Mutex<OptimizationService>> = Arc::new(Mutex::new(OptimizationService::new()));
}

#[derive(Serialize)]
pub struct PlatformInfo {
    pub os: String,
    pub version: String,
    pub arch: String,
}

#[command]
pub async fn get_available_optimizations() -> Result<Vec<OptimizationCategory>, String> {
    let service = OPTIMIZATION_SERVICE.lock().map_err(|e| e.to_string())?;
    service
        .get_available_optimizations()
        .map_err(|e| e.to_string())
}

#[command]
pub async fn apply_optimization(optimization_id: String) -> Result<OptimizationResult, String> {
    let service = OPTIMIZATION_SERVICE.lock().map_err(|e| e.to_string())?;
    service
        .apply_optimization(&optimization_id)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn revert_optimization(optimization_id: String) -> Result<OptimizationResult, String> {
    let service = OPTIMIZATION_SERVICE.lock().map_err(|e| e.to_string())?;
    service
        .revert_optimization(&optimization_id)
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_current_platform() -> PlatformInfo {
    let os = if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else {
        "Unknown"
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else if cfg!(target_arch = "aarch64") {
        "ARM64"
    } else {
        "Unknown"
    };

    // Get OS version for Windows
    let version = if cfg!(target_os = "windows") {
        // Try to get Windows version
        match std::process::Command::new("cmd")
            .args(&["/C", "ver"])
            .output()
        {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.contains("Windows 11") {
                    "11".to_string()
                } else if output_str.contains("Windows 10") {
                    "10".to_string()
                } else {
                    "Unknown".to_string()
                }
            }
            Err(_) => "Unknown".to_string(),
        }
    } else {
        "Unknown".to_string()
    };

    PlatformInfo {
        os: os.to_string(),
        version,
        arch: arch.to_string(),
    }
}
