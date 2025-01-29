use std::num::NonZeroUsize;
use sysinfo::System;

/// Struttura per le informazioni sulla memoria
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
}

/// Ottiene il numero di core CPU
pub fn get_cpu_count() -> NonZeroUsize {
    std::thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap())
}

/// Ottiene le informazioni sulla memoria del sistema
pub fn get_memory_info() -> MemoryInfo {
    let mut sys = System::new_all();
    sys.refresh_memory();

    MemoryInfo {
        total: sys.total_memory(),
        used: sys.used_memory(),
        free: sys.free_memory(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_count() {
        let count = get_cpu_count();
        assert!(count.get() > 0);
    }

    #[test]
    fn test_memory_info() {
        let info = get_memory_info();
        assert!(info.total > 0);
        assert!(info.used <= info.total);
        assert_eq!(info.free, info.total - info.used);
    }
}
