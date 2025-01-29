use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub utilization: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub memory_usage_percentage: f32,
    pub temperature: Option<f32>,
    pub power_usage: Option<f32>,
    pub clock_speed: Option<u32>,
    pub memory_clock: Option<u32>,
    pub driver_version: Option<String>,
    pub is_nvidia: bool,
    pub is_amd: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuStats {
    pub gpus: Vec<GpuInfo>,
    pub total_vram_used: u64,
    pub total_vram: u64,
    pub average_utilization: f32,
}

impl Default for GpuInfo {
    fn default() -> Self {
        Self {
            name: "Unknown GPU".to_string(),
            vendor: "Unknown".to_string(),
            utilization: 0.0,
            memory_used: 0,
            memory_total: 0,
            memory_usage_percentage: 0.0,
            temperature: None,
            power_usage: None,
            clock_speed: None,
            memory_clock: None,
            driver_version: None,
            is_nvidia: false,
            is_amd: false,
        }
    }
}

impl Default for GpuStats {
    fn default() -> Self {
        Self {
            gpus: Vec::new(),
            total_vram_used: 0,
            total_vram: 0,
            average_utilization: 0.0,
        }
    }
}
