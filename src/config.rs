use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxidianConfig {
    pub default_vault_path: Option<String>,
    pub theme: String,
    pub editor: String,
    pub show_backlinks: bool,
    pub relative_line_numbers: bool,
}

impl Default for OxidianConfig {
    fn default() -> Self {
        Self {
            default_vault_path: None,
            theme: "Catppuccin Mocha".to_string(),
            editor: "nvim".to_string(),
            show_backlinks: true,
            relative_line_numbers: true,
        }
    }
}

impl OxidianConfig {
    pub fn config_path() -> PathBuf {
        let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from(".config"));
        base.join("oxidian").join("config.json")
    }

    pub fn load() -> Option<Self> {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(cfg) = serde_json::from_str::<OxidianConfig>(&content) {
                    return Some(cfg);
                }
            }
        }
        None
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, json)?;
        Ok(())
    }
}
