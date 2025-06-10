use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::System;

lazy_static::lazy_static! {
    pub static ref SYSTEM: Arc<Mutex<System>> = Arc::new(Mutex::new(System::new_all()));
}

pub const REFRESH_INTERVAL: Duration = Duration::from_millis(100);

/// Get a reference to the shared system instance
pub fn get_system() -> &'static Arc<Mutex<System>> {
    &SYSTEM
}

/// Refresh the system and return a lock guard
pub fn refresh_and_lock() -> Result<std::sync::MutexGuard<'static, System>, String> {
    let mut system = SYSTEM
        .lock()
        .map_err(|e| format!("Failed to lock system: {}", e))?;
    system.refresh_all();
    Ok(system)
}
