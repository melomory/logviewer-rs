use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: "en".into(),
        }
    }
}
