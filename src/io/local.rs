use crate::core::parser::{LogParser, ShortLogParser};

pub fn load_local_logs(path: &str) -> anyhow::Result<Vec<crate::core::models::LogEntry>> {
    let parser: Box<dyn LogParser> = Box::new(ShortLogParser);
    parser.parse_file(path)
}
