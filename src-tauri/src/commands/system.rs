use sysinfo::System;
use tauri::command;

use crate::models::system_stats::{GenericData, SystemStats};

#[command]
pub fn get_system_stats() -> std::result::Result<SystemStats, String> {
    let mut system = System::new_all();
    system.refresh_all();

    let uptime = System::uptime();
    let days = uptime / (24 * 3600);
    let hours = (uptime % (24 * 3600)) / 3600;
    let minutes = (uptime % 3600) / 60;

    let uptime_str = if days > 0 {
        format!("{} days, {} hours", days, hours)
    } else if hours > 0 {
        format!("{} hours, {} minutes", hours, minutes)
    } else {
        format!("{} minutes", minutes)
    };

    let generic_data = vec![GenericData {
        title: "OS".to_string(),
        value: format!("{} {}",
                       System::name().unwrap_or("Unknown".to_string()),
                       System::os_version().unwrap_or("Unknown".to_string())
        ),
    },
                            GenericData {
                                title: "Uptime".to_string(),
                                value: uptime_str,
                            },
                            GenericData {
                                title: "CPU Cores".to_string(),
                                value: system.cpus().len().to_string(),
                            }, GenericData {
            title: "Hostname".to_string(),
            value: System::host_name().unwrap_or("Unknown".to_string()),
        },
    ];

    Ok(SystemStats {
        title: "System Info".to_string(),
        percentage: None,
        progress_data: None,
        generic_data: Some(generic_data),
    })
}
