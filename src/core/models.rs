use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct LogEntry {
    pub timestamp: Option<DateTime<FixedOffset>>,
    pub level: Option<LogLevel>,
    pub message: Option<String>,
    pub logger: Option<String>,
    pub process_id: Option<String>,
    pub trace_id: Option<String>,
    pub user: Option<String>,
    pub tenant: Option<String>,
    pub version: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
