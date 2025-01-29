pub mod loaded_module;
pub mod time;
pub mod bytes;
pub mod system;

pub use bytes::{format_bytes, format_bytes_per_second};
pub use system::{get_cpu_count, get_memory_info};
// Re-export delle funzioni più utilizzate
pub use time::{format_duration, format_milliseconds, format_run_time};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexports() {
        let time = format_run_time(3600);
        assert_eq!(time, "1h 0m 0s");

        let bytes = format_bytes(1024);
        assert_eq!(bytes, "1.00 KB");

        let speed = format_bytes_per_second(1048576);
        assert_eq!(speed, "1.00 MB/s");
    }
}
