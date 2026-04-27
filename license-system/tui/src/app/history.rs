use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokedLicense {
    pub owner: String,
    pub product_id: String,
    pub expires_at: i64,
    pub revoked_at: i64,
    pub revoke_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseHistory {
    pub revoked_licenses: Vec<RevokedLicense>,
}

impl LicenseHistory {
    pub fn load() -> Self {
        let path = Self::history_path();
        
        if let Ok(content) = fs::read_to_string(&path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::history_path();
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        
        Ok(())
    }

    pub fn add_revoked(&mut self, license: RevokedLicense) {
        self.revoked_licenses.push(license);
    }

    fn history_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home)
            .join(".config")
            .join("license-tui")
            .join("revoked_history.json")
    }
}

impl Default for LicenseHistory {
    fn default() -> Self {
        Self {
            revoked_licenses: Vec::new(),
        }
    }
}
