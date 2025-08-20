use std::{fs, path::PathBuf};

use directories::BaseDirs;

use crate::config::schema::Config;

pub fn config_path() -> PathBuf {
    let base = BaseDirs::new().unwrap();
    base.home_dir().join(".logviewerrc")
}

pub fn load_or_create_default() -> anyhow::Result<Config> {
    let path = config_path();
    if path.exists() {
        let data = fs::read_to_string(&path)?;
        let cfg: Config = toml::from_str(&data)?;
        Ok(cfg)
    } else {
        let config = Config::default();
        fs::write(&path, toml::to_string(&config)?)?;
        Ok(config)
    }
}
