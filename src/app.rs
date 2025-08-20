use crate::config::schema::Config;

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> anyhow::Result<Self> {
        Ok(Self { config })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        println!("App is running!");
        Ok(())
    }
}
