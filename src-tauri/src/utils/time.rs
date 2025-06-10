use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TimeError {
    #[error("Invalid time value: {0}")]
    InvalidTime(String),
}

type Result<T> = std::result::Result<T, TimeError>;

/// Formatta il tempo di esecuzione in una stringa leggibile
///
/// # Arguments
/// * `seconds` - Il numero di secondi da formattare
///
/// # Returns
/// Una stringa formattata nel formato "Xd Yh Zm Ws"
pub fn format_run_time(seconds: u64) -> String {
    let duration = Duration::from_secs(seconds);
    format_duration(duration)
}

/// Formatta una Duration in una stringa leggibile
///
/// # Arguments
/// * `duration` - La Duration da formattare
///
/// # Returns
/// Una stringa formattata nel formato "Xd Yh Zm Ws"
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();

    let days = total_seconds / (24 * 3600);
    let hours = (total_seconds % (24 * 3600)) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    let mut parts = Vec::new();

    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 || !parts.is_empty() {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 || !parts.is_empty() {
        parts.push(format!("{}m", minutes));
    }
    parts.push(format!("{}s", seconds));

    parts.join(" ")
}

/// Converte una stringa di tempo in secondi
///
/// # Arguments
/// * `time_str` - Stringa nel formato "Xd Yh Zm Ws"
///
/// # Returns
/// Il numero di secondi o un errore se il formato non è valido
pub fn parse_time_string(time_str: &str) -> Result<u64> {
    let mut total_seconds = 0u64;
    let parts: Vec<&str> = time_str.split_whitespace().collect();

    for part in parts {
        let (value, unit) = part.split_at(part.len() - 1);
        let value: u64 = value.parse().map_err(|_| {
            TimeError::InvalidTime(format!("Invalid number in time string: {}", part))
        })?;

        match unit {
            "d" => total_seconds += value * 24 * 3600,
            "h" => total_seconds += value * 3600,
            "m" => total_seconds += value * 60,
            "s" => total_seconds += value,
            _ => {
                return Err(TimeError::InvalidTime(format!(
                    "Invalid time unit: {}",
                    unit
                )))
            }
        }
    }

    Ok(total_seconds)
}

/// Formatta una durata in millisecondi in una stringa leggibile
///
/// # Arguments
/// * `ms` - Il numero di millisecondi
///
/// # Returns
/// Una stringa formattata con l'unità più appropriata
pub fn format_milliseconds(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        format_duration(Duration::from_millis(ms))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_run_time() {
        assert_eq!(format_run_time(0), "0s");
        assert_eq!(format_run_time(61), "1m 1s");
        assert_eq!(format_run_time(3661), "1h 1m 1s");
        assert_eq!(format_run_time(90061), "1d 1h 1m 1s");
    }

    #[test]
    fn test_parse_time_string() {
        assert_eq!(parse_time_string("1d 2h 3m 4s").unwrap(), 93784);
        assert_eq!(parse_time_string("1h").unwrap(), 3600);
        assert_eq!(parse_time_string("5m 30s").unwrap(), 330);
    }

    #[test]
    fn test_parse_invalid_time() {
        assert!(parse_time_string("1x").is_err());
        assert!(parse_time_string("1d 2x").is_err());
        assert!(parse_time_string("abc").is_err());
    }

    #[test]
    fn test_format_milliseconds() {
        assert_eq!(format_milliseconds(500), "500ms");
        assert_eq!(format_milliseconds(1500), "1.5s");
        assert_eq!(format_milliseconds(61000), "1m 1s");
    }

    #[test]
    fn test_format_duration() {
        let duration = Duration::from_secs(93784);
        assert_eq!(format_duration(duration), "1d 2h 3m 4s");
    }
}
