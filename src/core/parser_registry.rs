use crate::core::{models::LogEntry, parser::LogParser};

pub struct ParserRegistry {
    parsers: Vec<Box<dyn LogParser + Send + Sync>>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        Self {
            parsers: Vec::new(),
        }
    }

    pub fn register<P: LogParser + Send + Sync + 'static>(&mut self, parser: P) {
        self.parsers.push(Box::new(parser));
    }

    pub fn parse_line(&self, line: &str) -> anyhow::Result<LogEntry> {
        for parser in &self.parsers {
            if parser.can_parse(line) {
                return parser.parse_line(line);
            }
        }
        anyhow::bail!("No parser matched this line: {}", line)
    }
}
