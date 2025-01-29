use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize, Clone)]
pub struct SystemStats {
    pub title: String,
    pub percentage: Option<f32>,
    pub progress_data: Option<Vec<ProgressData>>,
    pub generic_data: Option<Vec<GenericData>>,
}

impl SystemStats {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            percentage: None,
            progress_data: None,
            generic_data: None,
        }
    }

    pub fn with_percentage(mut self, percentage: f32) -> Self {
        self.percentage = Some(percentage);
        self
    }

    pub fn with_progress_data(mut self, data: Vec<ProgressData>) -> Self {
        self.progress_data = Some(data);
        self
    }

    pub fn with_generic_data(mut self, data: Vec<GenericData>) -> Self {
        self.generic_data = Some(data);
        self
    }

    pub fn add_progress_data(&mut self, data: ProgressData) {
        self.progress_data.get_or_insert_with(Vec::new).push(data);
    }

    pub fn add_generic_data(&mut self, data: GenericData) {
        self.generic_data.get_or_insert_with(Vec::new).push(data);
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ProgressData {
    pub title: String,
    pub value: f32,
    pub temperature: Option<f32>,
}

impl ProgressData {
    pub fn new(title: impl Into<String>, value: f32) -> Self {
        Self {
            title: title.into(),
            value: value.clamp(0.0, 100.0),
            temperature: None,
        }
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
}

impl fmt::Display for ProgressData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:.1}%", self.title, self.value)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct GenericData {
    pub title: String,
    pub value: String,
}

impl GenericData {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
        }
    }
}

impl fmt::Display for GenericData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.title, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_stats_builder() {
        let stats = SystemStats::new("Test")
            .with_percentage(50.0)
            .with_progress_data(vec![ProgressData::new("CPU", 75.0)])
            .with_generic_data(vec![GenericData::new("Memory", "8GB")]);

        assert_eq!(stats.title, "Test");
        assert_eq!(stats.percentage, Some(50.0));
        assert!(stats.progress_data.is_some());
        assert!(stats.generic_data.is_some());
    }

    #[test]
    fn test_progress_data_clamp() {
        let data = ProgressData::new("Test", 150.0);
        assert_eq!(data.value, 100.0);

        let data = ProgressData::new("Test", -10.0);
        assert_eq!(data.value, 0.0);
    }

    #[test]
    fn test_data_display() {
        let progress = ProgressData::new("CPU", 75.5);
        assert_eq!(progress.to_string(), "CPU: 75.5%");

        let generic = GenericData::new("Memory", "8GB");
        assert_eq!(generic.to_string(), "Memory: 8GB");
    }

    #[test]
    fn test_add_data() {
        let mut stats = SystemStats::new("Test");
        stats.add_progress_data(ProgressData::new("CPU", 50.0));
        stats.add_generic_data(GenericData::new("Memory", "8GB"));

        assert!(stats.progress_data.is_some());
        assert!(stats.generic_data.is_some());
        assert_eq!(stats.progress_data.unwrap().len(), 1);
        assert_eq!(stats.generic_data.unwrap().len(), 1);
    }
}
