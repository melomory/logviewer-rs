use crate::{
    config::schema::Config,
    core::{parser::ShortLogParser, parser_registry::ParserRegistry},
};

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        Ok(Self { config })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        println!("App is running!");

        let mut registry = ParserRegistry::new();
        registry.register(ShortLogParser);
        //registry.register(StdLogParser);

        let line = r#"{"t":"2025-08-21 13:45:42.344+04:00","pid":"13452+56","l":"Warn","mt":"Client not found"}"#;

        let entry = registry.parse_line(line)?;
        println!("Parsed: {:?}", entry);

        Ok(())
    }
}
