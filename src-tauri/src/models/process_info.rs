use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub parent_pid: Option<i32>,
    pub session_id: i32,
    pub user: String,
    pub status: String,
    pub cpu: String,
    pub memory: String,
    pub gpu_usage: String,
    pub network: String,
    pub disk_io: String,
    pub env_vars: Vec<String>,
    pub children_processes: Vec<i32>,
}

impl ProcessInfo {
    pub fn new(pid: i32) -> Self {
        Self {
            pid,
            name: String::new(),
            parent_pid: None,
            session_id: 0,
            user: String::new(),
            status: String::new(),
            cpu: String::new(),
            memory: String::new(),
            gpu_usage: "N/A".to_string(),
            network: String::new(),
            disk_io: String::new(),
            env_vars: Vec::new(),
            children_processes: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_parent_pid(mut self, parent_pid: Option<i32>) -> Self {
        self.parent_pid = parent_pid;
        self
    }

    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = user.into();
        self
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = status.into();
        self
    }

    pub fn has_children(&self) -> bool {
        !self.children_processes.is_empty()
    }

    pub fn is_system_process(&self) -> bool {
        self.user.to_lowercase().contains("system") || self.user.to_lowercase().contains("root")
    }
}

impl fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Process {} (PID: {})", self.name, self.pid)?;
        if let Some(ppid) = self.parent_pid {
            write!(f, ", Parent PID: {}", ppid)?;
        }
        write!(f, ", User: {}, Status: {}", self.user, self.status)?;
        write!(f, ", CPU: {}, Memory: {}", self.cpu, self.memory)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessFilter {
    pub name: Option<String>,
    pub user: Option<String>,
    pub status: Option<ProcessStatus>,
    pub min_cpu: Option<f32>,
    pub min_memory: Option<u64>,
}

impl ProcessFilter {
    pub fn new() -> Self {
        Self {
            name: None,
            user: None,
            status: None,
            min_cpu: None,
            min_memory: None,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn with_status(mut self, status: ProcessStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_min_cpu(mut self, min_cpu: f32) -> Self {
        self.min_cpu = Some(min_cpu);
        self
    }

    pub fn with_min_memory(mut self, min_memory: u64) -> Self {
        self.min_memory = Some(min_memory);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ProcessStatus {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Unknown,
}

impl From<&str> for ProcessStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "running" => Self::Running,
            "sleeping" | "idle" => Self::Sleeping,
            "stopped" | "suspended" => Self::Stopped,
            "zombie" | "defunct" => Self::Zombie,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_info_builder() {
        let info = ProcessInfo::new(1234)
            .with_name("test")
            .with_user("testuser")
            .with_status("running");

        assert_eq!(info.pid, 1234);
        assert_eq!(info.name, "test");
        assert_eq!(info.user, "testuser");
        assert_eq!(info.status, "running");
    }

    #[test]
    fn test_process_filter_builder() {
        let filter = ProcessFilter::new()
            .with_name("test")
            .with_user("testuser")
            .with_status(ProcessStatus::Running)
            .with_min_cpu(1.0)
            .with_min_memory(1024);

        assert_eq!(filter.name, Some("test".to_string()));
        assert_eq!(filter.user, Some("testuser".to_string()));
        assert_eq!(filter.status, Some(ProcessStatus::Running));
        assert_eq!(filter.min_cpu, Some(1.0));
        assert_eq!(filter.min_memory, Some(1024));
    }

    #[test]
    fn test_process_status_from_str() {
        assert_eq!(ProcessStatus::from("running"), ProcessStatus::Running);
        assert_eq!(ProcessStatus::from("SLEEPING"), ProcessStatus::Sleeping);
        assert_eq!(ProcessStatus::from("stopped"), ProcessStatus::Stopped);
        assert_eq!(ProcessStatus::from("zombie"), ProcessStatus::Zombie);
        assert_eq!(ProcessStatus::from("invalid"), ProcessStatus::Unknown);
    }

    #[test]
    fn test_process_info_display() {
        let info = ProcessInfo::new(1234)
            .with_name("test")
            .with_parent_pid(Some(1000))
            .with_user("testuser")
            .with_status("running");

        let display = info.to_string();
        assert!(display.contains("test"));
        assert!(display.contains("1234"));
        assert!(display.contains("1000"));
        assert!(display.contains("testuser"));
        assert!(display.contains("running"));
    }
}
