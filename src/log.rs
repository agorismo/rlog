use crate::cli::LogLevel;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub raw: String,
    pub level: LogLevel,
}

pub struct LogStore {
    pub entries: Vec<LogEntry>,
}

impl LogStore {
    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let level = Self::detect_level(&line);
            entries.push(LogEntry { raw: line, level });
        }

        Ok(Self { entries })
    }

    fn detect_level(line: &str) -> LogLevel {
        let line_upper = line.to_uppercase();
        if line_upper.contains("ERROR") || line_upper.contains("ERR") {
            LogLevel::Error
        } else if line_upper.contains("WARN") || line_upper.contains("WARNING") {
            LogLevel::Warn
        } else if line_upper.contains("INFO") {
            LogLevel::Info
        } else {
            LogLevel::Debug
        }
    }
}