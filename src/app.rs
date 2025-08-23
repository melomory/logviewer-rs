use crate::{
    config::schema::Config,
    core::{models::LogEntry, parser::ShortLogParser, parser_registry::ParserRegistry},
    io::local,
};

pub struct App {
    config: Config,
    logs: Vec<LogEntry>,
}

impl App {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        Ok(Self {
            config,
            logs: Vec::new(),
        })
    }

    pub fn load_logs(&mut self, path: &str) -> anyhow::Result<()> {
        self.logs = local::load_local_logs(path)?;
        Ok(())
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        // For now just output the result to the console (without TUI).
        for entry in &self.logs {
            println!("[{:?}] {:?} - {:?}", entry.level, entry.timestamp, entry.message);
        }

        Ok(())
    }
}
