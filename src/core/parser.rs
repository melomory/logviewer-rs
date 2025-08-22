use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use serde::Deserialize;
use serde_json::Value;

use crate::core::models::{LogEntry, LogLevel};

pub trait LogParser {
    fn parse_line(&self, line: &str) -> anyhow::Result<LogEntry>;
    fn parse_file(&self, path: &str) -> anyhow::Result<Vec<LogEntry>>;
    fn can_parse(&self, line: &str) -> bool;
}

#[derive(Debug, Clone, Deserialize)]
struct RawShortLog {
    #[serde(rename = "t")]
    timestamp: Option<String>,
    #[serde(rename = "l")]
    level: Option<String>,
    #[serde(rename = "mt")]
    message_template: Option<String>,
    #[serde(rename = "lg")]
    logger: Option<String>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub struct ShortLogParser;

impl LogParser for ShortLogParser {
    fn parse_line(&self, line: &str) -> anyhow::Result<LogEntry> {
        let raw: RawShortLog = serde_json::from_str(line)?;

        Ok(LogEntry {
            timestamp: raw
                .timestamp
                .as_deref()
                .and_then(|s| chrono::DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.3f%z").ok()),
            level: raw.level.as_deref().map(|s| match s.to_lowercase().as_str() {
                "info" => LogLevel::Info,
                "warn" => LogLevel::Warn,
                "error" => LogLevel::Error,
                "debug" => LogLevel::Debug,
                "trace" => LogLevel::Trace,
                _ => LogLevel::Info,
            }),
            message: raw.message_template,
            logger: raw.logger,
            process_id: raw.extra.get("pid").map(|v| v.to_string()),
            trace_id: raw.extra.get("tr").map(|v| v.to_string()),
            user: raw.extra.get("un").map(|v| v.to_string()),
            tenant: raw.extra.get("tn").map(|v| v.to_string()),
            version: raw.extra.get("v").map(|v| v.to_string()),
            extra: raw.extra,
        })
    }

    fn parse_file(&self, path: &str) -> anyhow::Result<Vec<LogEntry>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line_result in reader.lines() {
            let line = line_result?;
            if let Ok(entry) = self.parse_line(&line) {
                entries.push(entry);
            } else {
                log::warn!("Could not parse line: {}", line);
            }
        }
        Ok(entries)
    }

    fn can_parse(&self, line: &str) -> bool {
        line.contains("\"t\"") && line.contains("\"mt\"")
    }
}
