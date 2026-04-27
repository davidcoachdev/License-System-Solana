use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: String,
    pub network: String,
}

impl Config {
    pub fn load() -> Self {
        let path = Self::config_path();
        
        if let Ok(content) = fs::read_to_string(&path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        
        Ok(())
    }

    fn config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home)
            .join(".config")
            .join("license-tui")
            .join("config.json")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "dcdev".to_string(),
            network: "localnet".to_string(),
        }
    }
}
