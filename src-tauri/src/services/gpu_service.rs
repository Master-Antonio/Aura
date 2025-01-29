use crate::models::gpu_info::{GpuInfo, GpuStats};
use anyhow::Result;
use sysinfo::System;

pub struct GpuService {
    system: System,
}

impl GpuService {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    pub fn get_gpu_stats(&mut self) -> Result<GpuStats> {
        self.system.refresh_all();
        let mut gpus = Vec::new();

        // Try to get NVIDIA GPU info
        if let Ok(nvidia_gpus) = self.get_nvidia_gpus() {
            gpus.extend(nvidia_gpus);
        }

        // Try to get AMD GPU info
        if let Ok(amd_gpus) = self.get_amd_gpus() {
            gpus.extend(amd_gpus);
        }

        // Fallback to generic GPU detection
        if gpus.is_empty() {
            gpus.extend(self.get_generic_gpus()?);
        }

        let total_vram = gpus.iter().map(|g| g.memory_total).sum();
        let total_vram_used = gpus.iter().map(|g| g.memory_used).sum();
        let average_utilization = if !gpus.is_empty() {
            gpus.iter().map(|g| g.utilization).sum::<f32>() / gpus.len() as f32
        } else {
            0.0
        };
        Ok(GpuStats {
            gpus,
            total_vram_used,
            total_vram,
            average_utilization,
        })
    }
    #[cfg(feature = "nvml-wrapper")]
    fn get_nvidia_gpus(&self) -> Result<Vec<GpuInfo>> {
        use nvml_wrapper::Nvml;

        let nvml = Nvml::init()?;
        let device_count = nvml.device_count()?;
        let mut gpus = Vec::new();

        for i in 0..device_count {
            let device = nvml.device_by_index(i)?;
            let name = device.name()?;
            let memory_info = device.memory_info()?;
            let utilization = device.utilization_rates()?.gpu;
            let temperature = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu).ok();
            let power_usage = device.power_usage().ok().map(|p| p as f32 / 1000.0);
            let clock_speed = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics).ok();
            let memory_clock = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Memory).ok();
            let driver_version = nvml.sys_driver_version().ok();

            gpus.push(GpuInfo {
                name,
                vendor: "NVIDIA".to_string(),
                utilization: utilization as f32,
                memory_used: memory_info.used,
                memory_total: memory_info.total,
                memory_usage_percentage: (memory_info.used as f32 / memory_info.total as f32) * 100.0,
                temperature: temperature.map(|t| t as f32),
                power_usage,
                clock_speed,
                memory_clock,
                driver_version,
                is_nvidia: true,
                is_amd: false,
            });
        }

        Ok(gpus)
    }
    #[cfg(not(feature = "nvml-wrapper"))]
    fn get_nvidia_gpus(&self) -> Result<Vec<GpuInfo>> {
        Ok(Vec::new())
    }

    fn get_amd_gpus(&self) -> Result<Vec<GpuInfo>> {
        // AMD GPU detection using WMI on Windows or lspci on Linux
        #[cfg(target_os = "windows")]
        {
            self.get_amd_gpus_windows()
        }
        #[cfg(target_os = "linux")]
        {
            self.get_amd_gpus_linux()
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            Ok(Vec::new())
        }
    }

    #[cfg(target_os = "windows")]
    fn get_amd_gpus_windows(&self) -> Result<Vec<GpuInfo>> {
        // Implement AMD GPU detection using Windows APIs
        // This is a simplified implementation
        Ok(Vec::new())
    }

    #[cfg(target_os = "linux")]
    fn get_amd_gpus_linux(&self) -> Result<Vec<GpuInfo>> {
        // Implement AMD GPU detection using sysfs
        use std::fs;
        let mut gpus = Vec::new();

        // Check for AMD GPUs in /sys/class/drm/
        if let Ok(entries) = fs::read_dir("/sys/class/drm/") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("card") && !name.contains("-") {
                        // Try to read GPU info from sysfs
                        let vendor_path = path.join("device/vendor");
                        let device_path = path.join("device/device");

                        if let (Ok(vendor), Ok(device)) = (
                            fs::read_to_string(&vendor_path),
                            fs::read_to_string(&device_path),
                        ) {
                            let vendor_id = vendor.trim();
                            let device_id = device.trim();

                            if vendor_id == "0x1002" { // AMD vendor ID
                                gpus.push(GpuInfo {
                                    name: format!("AMD GPU ({})", device_id),
                                    vendor: "AMD".to_string(),
                                    is_amd: true,
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(gpus)
    }

    fn get_generic_gpus(&self) -> Result<Vec<GpuInfo>> {        // Use wgpu for cross-platform GPU detection
        use wgpu::{Backends, Instance, InstanceDescriptor};

        let _instance = Instance::new(&InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let mut gpus = Vec::new();

        // This is a simplified approach - in a real implementation you'd need async handling
        // For now, we'll create a basic GPU info
        gpus.push(GpuInfo {
            name: "Generic GPU".to_string(),
            vendor: "Unknown".to_string(),
            ..Default::default()
        });

        Ok(gpus)
    }
}

impl Default for GpuService {
    fn default() -> Self {
        Self::new()
    }
}
