use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub is_applied: bool,
    pub is_reversible: bool,
    pub requires_admin: bool,
    pub risk_level: RiskLevel,
    pub platform: Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationCategory {
    pub name: String,
    pub items: Vec<OptimizationItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub success: bool,
    pub message: String,
    pub needs_restart: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub os: String,
    pub version: String,
    pub arch: String,
}
