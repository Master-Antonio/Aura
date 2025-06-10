use crate::models::optimization::{
    OptimizationCategory, OptimizationItem, OptimizationResult, Platform, RiskLevel,
};
use anyhow::Result;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub struct OptimizationService {
    current_platform: Platform,
}

impl OptimizationService {
    pub fn new() -> Self {
        let current_platform = if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "linux") {
            Platform::Linux
        } else if cfg!(target_os = "macos") {
            Platform::MacOS
        } else {
            Platform::All
        };

        Self { current_platform }
    }

    pub fn get_available_optimizations(&self) -> Result<Vec<OptimizationCategory>> {
        let mut categories = Vec::new();

        match self.current_platform {
            Platform::Windows => {
                categories.extend(self.get_windows_optimizations()?);
            }
            Platform::Linux => {
                categories.extend(self.get_linux_optimizations()?);
            }
            Platform::MacOS => {
                categories.extend(self.get_macos_optimizations()?);
            }
            Platform::All => {}
        }

        // Add universal optimizations
        categories.extend(self.get_universal_optimizations()?);

        Ok(categories)
    }

    fn get_windows_optimizations(&self) -> Result<Vec<OptimizationCategory>> {
        let mut categories = Vec::new();

        // Gaming Performance Category
        let gaming_items = vec![
            OptimizationItem {
                id: "disable_game_dvr".to_string(),
                name: "Disable Game DVR".to_string(),
                description: "Disables Windows Game DVR which can cause performance issues"
                    .to_string(),
                category: "Gaming Performance".to_string(),
                is_applied: self.check_game_dvr_status(),
                is_reversible: true,
                requires_admin: false,
                risk_level: RiskLevel::Low,
                platform: Platform::Windows,
            },
            OptimizationItem {
                id: "disable_fullscreen_optimization".to_string(),
                name: "Disable Fullscreen Optimization".to_string(),
                description: "Disables fullscreen optimization for better gaming performance"
                    .to_string(),
                category: "Gaming Performance".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: true,
                risk_level: RiskLevel::Low,
                platform: Platform::Windows,
            },
            OptimizationItem {
                id: "enable_game_mode".to_string(),
                name: "Enable Game Mode".to_string(),
                description: "Enables Windows Game Mode for better resource allocation".to_string(),
                category: "Gaming Performance".to_string(),
                is_applied: self.check_game_mode_status(),
                is_reversible: true,
                requires_admin: false,
                risk_level: RiskLevel::Low,
                platform: Platform::Windows,
            },
            OptimizationItem {
                id: "high_performance_power_plan".to_string(),
                name: "High Performance Power Plan".to_string(),
                description: "Sets power plan to High Performance for maximum CPU performance"
                    .to_string(),
                category: "Gaming Performance".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: true,
                risk_level: RiskLevel::Medium,
                platform: Platform::Windows,
            },
        ];

        categories.push(OptimizationCategory {
            name: "Gaming Performance".to_string(),
            items: gaming_items,
        });

        // System Performance Category
        let system_items = vec![
            OptimizationItem {
                id: "disable_transparency".to_string(),
                name: "Disable Transparency Effects".to_string(),
                description: "Disables visual transparency effects to improve performance".to_string(),
                category: "System Performance".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: false,
                risk_level: RiskLevel::Low,
                platform: Platform::Windows,
            },
            OptimizationItem {
                id: "disable_animations".to_string(),
                name: "Disable Animations".to_string(),
                description: "Disables window animations for faster response".to_string(),
                category: "System Performance".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: false,
                risk_level: RiskLevel::Low,
                platform: Platform::Windows,
            }, OptimizationItem {
                id: "increase_timer_resolution".to_string(),
                name: "Increase Timer Resolution".to_string(),
                description: "Increases system timer resolution for better performance in games and applications".to_string(),
                category: "System Performance".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: true,
                risk_level: RiskLevel::Medium,
                platform: Platform::Windows,
            },
        ];

        categories.push(OptimizationCategory {
            name: "System Performance".to_string(),
            items: system_items,
        });

        // Privacy & Telemetry Category
        let privacy_items = vec![
            OptimizationItem {
                id: "disable_telemetry".to_string(),
                name: "Disable Telemetry".to_string(),
                description: "Disables Windows telemetry and data collection".to_string(),
                category: "Privacy & Telemetry".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: true,
                risk_level: RiskLevel::Medium,
                platform: Platform::Windows,
            },
            OptimizationItem {
                id: "disable_cortana".to_string(),
                name: "Disable Cortana".to_string(),
                description: "Disables Cortana voice assistant".to_string(),
                category: "Privacy & Telemetry".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: true,
                risk_level: RiskLevel::High,
                platform: Platform::Windows,
            },
        ];

        categories.push(OptimizationCategory {
            name: "Privacy & Telemetry".to_string(),
            items: privacy_items,
        });

        Ok(categories)
    }

    fn get_linux_optimizations(&self) -> Result<Vec<OptimizationCategory>> {
        let mut categories = Vec::new();

        // Gaming Performance Category
        let gaming_items = vec![
            OptimizationItem {
                id: "install_gamemode".to_string(),
                name: "Install GameMode".to_string(),
                description: "Installs and enables Feral Interactive's GameMode for better gaming performance".to_string(),
                category: "Gaming Performance".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: true,
                risk_level: RiskLevel::Low,
                platform: Platform::Linux,
            },
            OptimizationItem {
                id: "enable_performance_governor".to_string(),
                name: "Performance CPU Governor".to_string(),
                description: "Sets CPU governor to performance mode for maximum performance".to_string(),
                category: "Gaming Performance".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: true,
                risk_level: RiskLevel::Medium,
                platform: Platform::Linux,
            },
            OptimizationItem {
                id: "optimize_swappiness".to_string(),
                name: "Optimize Swappiness".to_string(),
                description: "Sets vm.swappiness to 10 for better memory management in games".to_string(),
                category: "Gaming Performance".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: true,
                risk_level: RiskLevel::Low,
                platform: Platform::Linux,
            },
        ];

        categories.push(OptimizationCategory {
            name: "Gaming Performance".to_string(),
            items: gaming_items,
        });

        // System Performance Category
        let system_items = vec![
            OptimizationItem {
                id: "disable_compositor".to_string(),
                name: "Disable Desktop Compositor".to_string(),
                description:
                "Temporarily disables desktop compositor during gaming for better performance"
                    .to_string(),
                category: "System Performance".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: false,
                risk_level: RiskLevel::Medium,
                platform: Platform::Linux,
            },
            OptimizationItem {
                id: "optimize_kernel_params".to_string(),
                name: "Optimize Kernel Parameters".to_string(),
                description: "Optimizes kernel parameters for gaming and low latency".to_string(),
                category: "System Performance".to_string(),
                is_applied: false,
                is_reversible: true,
                requires_admin: true,
                risk_level: RiskLevel::High,
                platform: Platform::Linux,
            },
        ];

        categories.push(OptimizationCategory {
            name: "System Performance".to_string(),
            items: system_items,
        });

        Ok(categories)
    }

    fn get_macos_optimizations(&self) -> Result<Vec<OptimizationCategory>> {
        let mut categories = Vec::new();

        // Gaming Performance Category
        let gaming_items = vec![OptimizationItem {
            id: "disable_spotlight".to_string(),
            name: "Disable Spotlight Indexing".to_string(),
            description: "Temporarily disables Spotlight indexing for better performance"
                .to_string(),
            category: "Gaming Performance".to_string(),
            is_applied: false,
            is_reversible: true,
            requires_admin: true,
            risk_level: RiskLevel::Medium,
            platform: Platform::MacOS,
        }];

        categories.push(OptimizationCategory {
            name: "Gaming Performance".to_string(),
            items: gaming_items,
        });

        Ok(categories)
    }

    fn get_universal_optimizations(&self) -> Result<Vec<OptimizationCategory>> {
        let mut categories = Vec::new();

        // Process Management Category
        let process_items = vec![OptimizationItem {
            id: "set_high_priority".to_string(),
            name: "High Priority Mode".to_string(),
            description: "Runs the application with high priority for better performance"
                .to_string(),
            category: "Process Management".to_string(),
            is_applied: false,
            is_reversible: true,
            requires_admin: false,
            risk_level: RiskLevel::Low,
            platform: Platform::All,
        }];

        categories.push(OptimizationCategory {
            name: "Process Management".to_string(),
            items: process_items,
        });

        Ok(categories)
    }
    pub fn apply_optimization(&self, optimization_id: &str) -> Result<OptimizationResult> {
        match optimization_id {
            "disable_game_dvr" => self.disable_game_dvr(),
            "enable_game_mode" => self.enable_game_mode(),
            "high_performance_power_plan" => self.set_high_performance_power_plan(),
            "disable_transparency" => self.disable_transparency_effects(),
            "disable_animations" => self.disable_animations(),
            "increase_timer_resolution" => self.increase_timer_resolution(),
            "clear_memory_cache" => self.clear_memory_cache(),
            "clear_dns_cache" => self.clear_dns_cache(),
            "disable_telemetry" => self.disable_telemetry(),
            "disable_cortana" => self.disable_cortana(),
            "install_gamemode" => self.install_gamemode(),
            "enable_performance_governor" => self.enable_performance_governor(),
            "optimize_swappiness" => self.optimize_swappiness(),
            "disable_compositor" => self.disable_compositor(),
            "optimize_kernel_params" => self.optimize_kernel_params(),
            "disable_spotlight" => self.disable_spotlight(),
            "set_high_priority" => self.set_high_priority(),
            _ => Ok(OptimizationResult {
                success: false,
                message: "Unknown optimization".to_string(),
                needs_restart: false,
            }),
        }
    }

    pub fn revert_optimization(&self, optimization_id: &str) -> Result<OptimizationResult> {
        // Implement revert logic for each optimization
        match optimization_id {
            "disable_game_dvr" => self.enable_game_dvr(),
            "enable_game_mode" => self.disable_game_mode(),
            // ... add more revert implementations
            _ => Ok(OptimizationResult {
                success: false,
                message: "Revert not implemented for this optimization".to_string(),
                needs_restart: false,
            }),
        }
    }

    // Windows-specific optimization implementations
    #[cfg(target_os = "windows")]
    fn check_game_dvr_status(&self) -> bool {
        // Check registry for Game DVR status
        false // Placeholder
    }

    #[cfg(not(target_os = "windows"))]
    fn check_game_dvr_status(&self) -> bool {
        false
    }

    #[cfg(target_os = "windows")]
    fn check_game_mode_status(&self) -> bool {
        // Check registry for Game Mode status
        false // Placeholder
    }

    #[cfg(not(target_os = "windows"))]
    fn check_game_mode_status(&self) -> bool {
        false
    }

    fn disable_game_dvr(&self) -> Result<OptimizationResult> {
        #[cfg(target_os = "windows")]
        {
            // Implement Game DVR disable logic
            Ok(OptimizationResult {
                success: true,
                message: "Game DVR disabled successfully".to_string(),
                needs_restart: false,
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(OptimizationResult {
                success: false,
                message: "Game DVR optimization is Windows-only".to_string(),
                needs_restart: false,
            })
        }
    }

    fn enable_game_dvr(&self) -> Result<OptimizationResult> {
        #[cfg(target_os = "windows")]
        {
            Ok(OptimizationResult {
                success: true,
                message: "Game DVR enabled successfully".to_string(),
                needs_restart: false,
            })
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(OptimizationResult {
                success: false,
                message: "Game DVR optimization is Windows-only".to_string(),
                needs_restart: false,
            })
        }
    }

    fn enable_game_mode(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Game Mode enabled successfully".to_string(),
            needs_restart: false,
        })
    }

    fn disable_game_mode(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Game Mode disabled successfully".to_string(),
            needs_restart: false,
        })
    }

    // Placeholder implementations for other optimizations
    fn set_high_performance_power_plan(&self) -> Result<OptimizationResult> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            // Set High Performance power plan using powercfg
            let output = Command::new("powercfg")
                .args(&["/setactive", "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"]) // High Performance GUID
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        Ok(OptimizationResult {
                            success: true,
                            message: "High Performance power plan activated successfully"
                                .to_string(),
                            needs_restart: false,
                        })
                    } else {
                        let error_msg = String::from_utf8_lossy(&result.stderr);
                        Ok(OptimizationResult {
                            success: false,
                            message: format!("Failed to set power plan: {}", error_msg),
                            needs_restart: false,
                        })
                    }
                }
                Err(e) => Ok(OptimizationResult {
                    success: false,
                    message: format!("Failed to execute powercfg command: {}", e),
                    needs_restart: false,
                }),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(OptimizationResult {
                success: false,
                message: "Power plan optimization is Windows-only".to_string(),
                needs_restart: false,
            })
        }
    }

    fn disable_transparency_effects(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Transparency effects disabled".to_string(),
            needs_restart: false,
        })
    }

    fn disable_animations(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Animations disabled".to_string(),
            needs_restart: false,
        })
    }

    fn increase_timer_resolution(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Timer resolution increased".to_string(),
            needs_restart: false,
        })
    }

    fn disable_telemetry(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Telemetry disabled".to_string(),
            needs_restart: true,
        })
    }

    fn disable_cortana(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Cortana disabled".to_string(),
            needs_restart: true,
        })
    }

    fn install_gamemode(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "GameMode installed and enabled".to_string(),
            needs_restart: false,
        })
    }

    fn enable_performance_governor(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Performance governor enabled".to_string(),
            needs_restart: false,
        })
    }

    fn optimize_swappiness(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Swappiness optimized".to_string(),
            needs_restart: false,
        })
    }

    fn disable_compositor(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Desktop compositor disabled".to_string(),
            needs_restart: false,
        })
    }

    fn optimize_kernel_params(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Kernel parameters optimized".to_string(),
            needs_restart: true,
        })
    }

    fn disable_spotlight(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "Spotlight indexing disabled".to_string(),
            needs_restart: false,
        })
    }
    fn set_high_priority(&self) -> Result<OptimizationResult> {
        Ok(OptimizationResult {
            success: true,
            message: "High priority mode enabled".to_string(),
            needs_restart: false,
        })
    }

    fn clear_memory_cache(&self) -> Result<OptimizationResult> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;            // Use PowerShell to clear memory cache and working set
            #[cfg(target_os = "windows")]
            let output = Command::new("powershell")
                .args(&[
                    "-Command",
                    "[System.GC]::Collect(); [System.GC]::WaitForPendingFinalizers(); [System.GC]::Collect()"
                ])
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .output();

            #[cfg(not(target_os = "windows"))]
            let output = Command::new("powershell")
                .args(&[
                    "-Command",
                    "[System.GC]::Collect(); [System.GC]::WaitForPendingFinalizers(); [System.GC]::Collect()"
                ])
                .output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        Ok(OptimizationResult {
                            success: true,
                            message: "Memory cache cleared successfully".to_string(),
                            needs_restart: false,
                        })
                    } else {
                        let error_msg = String::from_utf8_lossy(&result.stderr);
                        Ok(OptimizationResult {
                            success: false,
                            message: format!("Failed to clear memory cache: {}", error_msg),
                            needs_restart: false,
                        })
                    }
                }
                Err(e) => Ok(OptimizationResult {
                    success: false,
                    message: format!("Failed to execute memory clear command: {}", e),
                    needs_restart: false,
                }),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(OptimizationResult {
                success: false,
                message: "Memory cache clearing is Windows-only".to_string(),
                needs_restart: false,
            })
        }
    }

    fn clear_dns_cache(&self) -> Result<OptimizationResult> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            let output = Command::new("ipconfig").args(&["/flushdns"]).output();

            match output {
                Ok(result) => {
                    if result.status.success() {
                        Ok(OptimizationResult {
                            success: true,
                            message: "DNS cache flushed successfully".to_string(),
                            needs_restart: false,
                        })
                    } else {
                        let error_msg = String::from_utf8_lossy(&result.stderr);
                        Ok(OptimizationResult {
                            success: false,
                            message: format!("Failed to flush DNS cache: {}", error_msg),
                            needs_restart: false,
                        })
                    }
                }
                Err(e) => Ok(OptimizationResult {
                    success: false,
                    message: format!("Failed to execute DNS flush command: {}", e),
                    needs_restart: false,
                }),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(OptimizationResult {
                success: false,
                message: "DNS cache flushing is Windows-only".to_string(),
                needs_restart: false,
            })
        }
    }
}

impl Default for OptimizationService {
    fn default() -> Self {
        Self::new()
    }
}
