use crate::models::system_stats::{GenericData, ProgressData, SystemStats};
use std::{
    process::Command,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use sysinfo::Networks;
use tauri::command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const NETWORK_SAMPLE_INTERVAL: Duration = Duration::from_millis(1000);
const CACHE_DURATION: Duration = Duration::from_secs(2);
const BYTES_IN_MB: f64 = 1024.0 * 1024.0;

#[derive(Clone)]
struct NetworkInfo {
    download_speed: u64,
    upload_speed: u64,
    total_received: u64,
    total_transmitted: u64,
    interfaces: Vec<InterfaceInfo>,
}

#[derive(Clone)]
struct InterfaceInfo {
    name: String,
    received: u64,
    transmitted: u64,
    speed_down: u64,
    speed_up: u64,
    link_speed: Option<u64>, // in Mbps
    interface_type: String,
}

#[derive(Clone, Debug)]
struct NetworkAdapterInfo {
    name: String,
    speed: Option<u64>, // in Mbps
    interface_type: String,
    status: String,
}

#[cfg(target_os = "windows")]
fn get_network_adapters() -> Vec<NetworkAdapterInfo> {
    let mut adapters = Vec::new();    // Get ALL network adapter info using wmic (not just connected ones)
    #[cfg(target_os = "windows")]
    let output = Command::new("wmic")
        .args(&[
            "path",
            "win32_networkadapter",
            "get",
            "Name,Speed,AdapterType,NetConnectionStatus,MACAddress",
            "/format:csv",
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output();

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("wmic")
        .args(&[
            "path",
            "win32_networkadapter",
            "get",
            "Name,Speed,AdapterType,NetConnectionStatus,MACAddress",
            "/format:csv",
        ])
        .output();

    if let Ok(output) = output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();

        for line in lines.iter().skip(1) {
            // Skip header
            if !line.trim().is_empty() && line.contains(',') {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 6 {
                    let adapter_type = parts[1].trim();
                    let mac_address = parts[2].trim();
                    let name = parts[3].trim();
                    let status_code = parts[4].trim();
                    let speed = parts[5].trim();

                    // Skip virtual adapters and loopback
                    if name.is_empty()
                        || name.to_lowercase().contains("loopback")
                        || name.to_lowercase().contains("isatap")
                        || name.to_lowercase().contains("teredo")
                        || name.to_lowercase().contains("virtual")
                        || mac_address.is_empty()
                        || mac_address == "NULL"
                    {
                        continue;
                    }

                    let status = match status_code {
                        "2" => "Connected",
                        "7" => "Disconnected",
                        "0" => "Disabled",
                        _ => "Unknown",
                    };

                    let speed_mbps = if !speed.is_empty() && speed != "NULL" {
                        speed.parse::<u64>().ok().map(|s| s / 1_000_000) // Convert from bps to Mbps
                    } else {
                        None
                    };

                    let interface_type = if adapter_type.contains("Ethernet")
                        || name.to_lowercase().contains("ethernet")
                    {
                        "Ethernet".to_string()
                    } else if adapter_type.contains("Wireless")
                        || name.to_lowercase().contains("wi-fi")
                        || name.to_lowercase().contains("wireless")
                    {
                        "Wi-Fi".to_string()
                    } else if name.to_lowercase().contains("bluetooth") {
                        "Bluetooth".to_string()
                    } else {
                        "Other".to_string()
                    };

                    adapters.push(NetworkAdapterInfo {
                        name: name.to_string(),
                        speed: speed_mbps,
                        interface_type,
                        status: status.to_string(),
                    });
                }
            }
        }
    }

    adapters
}

#[cfg(not(target_os = "windows"))]
fn get_network_adapters() -> Vec<NetworkAdapterInfo> {
    Vec::new() // Placeholder for non-Windows systems
}

struct NetworkCache {
    info: Option<NetworkInfo>,
    last_update: Instant,
    previous_stats: Option<NetworkTotals>,
}

impl NetworkCache {
    fn new() -> Self {
        Self {
            info: None,
            last_update: Instant::now(),
            previous_stats: None,
        }
    }

    fn needs_update(&self) -> bool {
        self.info.is_none() || self.last_update.elapsed() >= CACHE_DURATION
    }
}

lazy_static::lazy_static! {
    static ref NETWORK_CACHE: Arc<Mutex<NetworkCache>> = Arc::new(Mutex::new(NetworkCache::new()));
}

#[derive(Clone)]
struct NetworkTotals {
    received: u64,
    transmitted: u64,
    timestamp: Instant,
}

fn get_network_totals(networks: &Networks) -> NetworkTotals {
    let mut total_received = 0;
    let mut total_transmitted = 0;

    for (_, data) in networks.iter() {
        total_received += data.received();
        total_transmitted += data.transmitted();
    }

    NetworkTotals {
        received: total_received,
        transmitted: total_transmitted,
        timestamp: Instant::now(),
    }
}

fn measure_network_speed(networks: &mut Networks, cache: &mut NetworkCache) -> NetworkInfo {
    let current_stats = get_network_totals(networks);
    let (download_speed, upload_speed) = if let Some(ref previous) = cache.previous_stats {
        let time_diff = current_stats
            .timestamp
            .duration_since(previous.timestamp)
            .as_secs_f64();
        if time_diff > 0.0 {
            // Prevent overflow by checking if current values are greater than previous
            let download = if current_stats.received >= previous.received {
                ((current_stats.received - previous.received) as f64 / time_diff) as u64
            } else {
                0 // Reset likely occurred
            };
            let upload = if current_stats.transmitted >= previous.transmitted {
                ((current_stats.transmitted - previous.transmitted) as f64 / time_diff) as u64
            } else {
                0 // Reset likely occurred
            };
            (download, upload)
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    };

    // Get network adapter information
    let adapters = get_network_adapters();
    // Get interface details with enhanced information
    let mut interfaces = Vec::new();
    for (interface_name, data) in networks.iter() {
        // Find matching adapter info
        let adapter_info = adapters.iter().find(|adapter| {
            adapter
                .name
                .to_lowercase()
                .contains(&interface_name.to_lowercase())
                || interface_name
                .to_lowercase()
                .contains(&adapter.name.to_lowercase())
                || interface_name
                == &adapter
                .name
                .replace("Intel(R) ", "")
                .replace("Realtek ", "")
        });

        // Calculate per-interface speeds if we have previous data
        let (interface_down_speed, interface_up_speed) = if cache.previous_stats.is_some() {
            let time_diff = current_stats
                .timestamp
                .duration_since(cache.previous_stats.as_ref().unwrap().timestamp)
                .as_secs_f64();
            if time_diff > 0.0 {
                let down_speed = ((data.received() as f64
                    - cache.previous_stats.as_ref().unwrap().received as f64)
                    / time_diff) as u64;
                let up_speed = ((data.transmitted() as f64
                    - cache.previous_stats.as_ref().unwrap().transmitted as f64)
                    / time_diff) as u64;
                (down_speed, up_speed)
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        };

        interfaces.push(InterfaceInfo {
            name: interface_name.clone(),
            received: data.received(),
            transmitted: data.transmitted(),
            speed_down: interface_down_speed,
            speed_up: interface_up_speed,
            link_speed: adapter_info.and_then(|a| a.speed),
            interface_type: adapter_info
                .map(|a| a.interface_type.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
        });
    }

    cache.previous_stats = Some(current_stats.clone());

    NetworkInfo {
        download_speed,
        upload_speed,
        total_received: current_stats.received,
        total_transmitted: current_stats.transmitted,
        interfaces,
    }
}

fn format_network_speed(bytes_per_sec: u64) -> String {
    let bytes = bytes_per_sec as f64;
    if bytes >= BYTES_IN_MB {
        format!("{:.2} MB/s", bytes / BYTES_IN_MB)
    } else {
        format!("{:.2} KB/s", bytes / 1024.0)
    }
}

fn format_bytes(bytes: u64) -> String {
    let bytes = bytes as f64;
    if bytes >= 1024.0 * 1024.0 * 1024.0 {
        format!("{:.2} GB", bytes / (1024.0 * 1024.0 * 1024.0))
    } else if bytes >= 1024.0 * 1024.0 {
        format!("{:.2} MB", bytes / (1024.0 * 1024.0))
    } else {
        format!("{:.2} KB", bytes / 1024.0)
    }
}

#[command]
pub fn get_network_stats() -> Result<SystemStats, String> {
    let mut cache = NETWORK_CACHE
        .lock()
        .map_err(|e| format!("Cache lock error: {}", e))?;

    if cache.needs_update() {
        let mut networks = Networks::new_with_refreshed_list();
        std::thread::sleep(NETWORK_SAMPLE_INTERVAL);
        networks.refresh(true);

        let network_info = measure_network_speed(&mut networks, &mut cache);
        cache.info = Some(network_info);
        cache.last_update = Instant::now();
    }

    let info = cache.info.as_ref().unwrap();

    // Calculate overall network usage percentage (based on typical home connection speeds)
    let typical_home_speed = 100.0 * BYTES_IN_MB; // 100 MB/s typical
    let total_usage = info.download_speed + info.upload_speed;
    let usage_percentage = ((total_usage as f64 / typical_home_speed) * 100.0).min(100.0) as f32; // Create progress data for ALL interfaces (both active and inactive)
    let mut progress_data = Vec::new();
    let adapters = get_network_adapters(); // Get all network adapters

    // Add all detected network adapters, not just active ones
    for adapter in &adapters {
        // Try to find matching sysinfo interface
        let sysinfo_interface = info.interfaces.iter().find(|iface| {
            iface
                .name
                .to_lowercase()
                .contains(&adapter.name.to_lowercase())
                || adapter
                .name
                .to_lowercase()
                .contains(&iface.name.to_lowercase())
        });

        let interface_percentage = if let Some(iface) = sysinfo_interface {
            let interface_total = iface.received + iface.transmitted;
            if info.total_received + info.total_transmitted > 0 {
                ((interface_total as f64 / (info.total_received + info.total_transmitted) as f64)
                    * 100.0) as f32
            } else {
                0.0
            }
        } else {
            0.0 // Interface exists but has no traffic data
        };
        // Create comprehensive interface title with all available information
        let interface_title = if let Some(speed) = adapter.speed {
            format!(
                "{} ({}) - {} Mbps [{}]",
                adapter.name, adapter.interface_type, speed, adapter.status
            )
        } else {
            format!(
                "{} ({}) [{}]",
                adapter.name, adapter.interface_type, adapter.status
            )
        };

        // Calculate more accurate interface usage based on traffic
        let interface_usage = if let Some(iface) = sysinfo_interface {
            let interface_total_traffic = iface.received + iface.transmitted;
            if interface_total_traffic > 0 && adapter.speed.is_some() {
                let speed_bps = adapter.speed.unwrap() * 1_000_000; // Convert Mbps to bps
                let recent_traffic = interface_total_traffic; // This would ideally be recent delta
                let traffic_percentage =
                    ((recent_traffic as f64 / speed_bps as f64) * 100.0).min(100.0);
                traffic_percentage as f32
            } else {
                interface_percentage
            }
        } else {
            0.0 // Interface exists but no traffic data
        };

        progress_data.push(ProgressData {
            title: interface_title,
            value: interface_usage,
            temperature: None,
        });
    }

    let generic_data = vec![
        GenericData {
            title: "Download Speed".to_string(),
            value: format_network_speed(info.download_speed),
        },
        GenericData {
            title: "Upload Speed".to_string(),
            value: format_network_speed(info.upload_speed),
        },
        GenericData {
            title: "Total Downloaded".to_string(),
            value: format_bytes(info.total_received),
        },
        GenericData {
            title: "Total Uploaded".to_string(),
            value: format_bytes(info.total_transmitted),
        },
        GenericData {
            title: "Active Interfaces".to_string(),
            value: info.interfaces.len().to_string(),
        },
    ];

    Ok(SystemStats {
        title: "Network".to_string(),
        percentage: Some(usage_percentage),
        progress_data: Some(progress_data),
        generic_data: Some(generic_data),
    })
}
