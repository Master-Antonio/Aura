// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod services;
mod shared;
mod ui;
mod utils;

use aura_lib::ui::window::setup_window_effects;

// Import local commands
use commands::cpu::get_cpu_stats;
use commands::gpu::get_gpu_stats;
use commands::memory::get_memory_stats;
use commands::network::get_network_stats;
use commands::optimization_commands::{
    apply_optimization, get_available_optimizations, get_current_platform, revert_optimization,
};
use commands::optimizations::{disable_game_dvr, optimize_time_resolution};
use commands::process::open_file_location;
use commands::processes::{
    boost_process_for_gaming, get_cpu_core_count, get_detailed_process_info, get_process_affinity,
    get_processes, get_running_processes, kill_process, resume_process, set_process_affinity,
    suspend_process,
};
use commands::resilient_monitor::{
    get_monitor_health, get_resilient_cpu_stats, get_resilient_memory_stats,
    get_resilient_network_stats, get_resilient_storage_stats, get_resilient_system_stats,
    reset_monitor_health,
};
use commands::storage::get_storage_stats;
use commands::system::get_system_stats;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            setup_window_effects(&window).expect("Failed to apply window effects");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_cpu_stats,
            get_memory_stats,
            get_storage_stats,
            get_network_stats,
            get_system_stats,
            get_resilient_cpu_stats,
            get_resilient_memory_stats,
            get_resilient_storage_stats,
            get_resilient_network_stats,
            get_resilient_system_stats,
            get_monitor_health,
            reset_monitor_health,
            get_detailed_process_info,
            get_processes,
            get_running_processes,
            boost_process_for_gaming,
            set_process_affinity,
            get_process_affinity,
            get_cpu_core_count,
            kill_process,
            suspend_process,
            resume_process,
            open_file_location,
            disable_game_dvr,
            optimize_time_resolution,
            get_gpu_stats,
            get_available_optimizations,
            apply_optimization,
            revert_optimization,
            get_current_platform,
        ])
        .run(tauri::generate_context!())
        .expect("Errore nell'avviare l'applicazione");
}
