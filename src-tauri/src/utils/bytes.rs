const KB: f64 = 1024.0;
const MB: f64 = KB * 1024.0;
const GB: f64 = MB * 1024.0;
const TB: f64 = GB * 1024.0;

/// Formatta un numero di bytes in una stringa leggibile
pub fn format_bytes(bytes: u64) -> String {
    let bytes = bytes as f64;
    match bytes {
        b if b >= TB => format!("{:.1} TB", b / TB),
        b if b >= GB => {
            let gb_value = b / GB;
            if gb_value >= 1000.0 {
                format!("{:.1} TB", gb_value / 1000.0)
            } else {
                format!("{:.0} GB", gb_value)
            }
        }
        b if b >= MB => {
            let mb_value = b / MB;
            if mb_value >= 1000.0 {
                format!("{:.0} GB", mb_value / 1000.0)
            } else {
                format!("{:.0} MB", mb_value)
            }
        }
        b if b >= KB => {
            let kb_value = b / KB;
            if kb_value >= 1000.0 {
                format!("{:.0} MB", kb_value / 1000.0)
            } else {
                format!("{:.0} KB", kb_value)
            }
        }
        b => format!("{:.0} B", b),
    }
}

/// Formatta una velocità in bytes/secondo
pub fn format_bytes_per_second(bytes: u64) -> String {
    format!("{}/s", format_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1500), "1.46 KB");
        assert_eq!(format_bytes(1500000), "1.43 MB");
        assert_eq!(format_bytes(1500000000), "1.40 GB");
    }

    #[test]
    fn test_format_bytes_per_second() {
        assert_eq!(format_bytes_per_second(1024), "1.00 KB/s");
        assert_eq!(format_bytes_per_second(1048576), "1.00 MB/s");
    }
} 