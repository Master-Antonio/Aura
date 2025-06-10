//! Aura library
//!
//! Questa libreria fornisce funzionalità per l'ottimizzazione automatica
//! delle prestazioni dei giochi e il monitoraggio del sistema.

pub mod commands;
pub mod models;
pub mod services;
pub mod shared;
pub mod ui;
pub mod utils;

// Re-export delle funzioni più utilizzate per semplificare l'accesso
pub use commands::{
    cpu::get_cpu_stats,
    gpu::get_gpu_stats,
    memory::get_memory_stats,
    network::get_network_stats,
    optimization_commands::{
        apply_optimization, get_available_optimizations, get_current_platform, revert_optimization,
    },
    optimizations::{disable_game_dvr, optimize_time_resolution},
    process::open_file_location,
    processes::{
        get_processes, get_running_processes, kill_process, resume_process, suspend_process,
    },
    storage::get_storage_stats,
    system::get_system_stats,
};

pub use models::{
    process_info::{ProcessFilter, ProcessInfo, ProcessStatus},
    system_stats::{GenericData, ProgressData, SystemStats},
};

pub use services::{
    process_control::{
        kill_process as kill_process_service, suspend_process as suspend_process_service,
    },
    process_info::{name, parent_pid, session_id, status, user},
};

pub use utils::{
    bytes::{format_bytes, format_bytes_per_second},
    time::{format_duration, format_milliseconds, format_run_time},
};

// Mobile entry point commented out due to compatibility issues
// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexports() {
        // Verifica che i re-export funzionino correttamente
        let time = format_run_time(3600);
        assert_eq!(time, "1h 0m 0s");

        let bytes = format_bytes(1024);
        assert_eq!(bytes, "1.00 KB");

        let speed = format_bytes_per_second(1048576);
        assert_eq!(speed, "1.00 MB/s");
    }
}
