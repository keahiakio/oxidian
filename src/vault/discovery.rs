use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObsidianConfig {
    pub vaults: HashMap<String, ObsidianVaultEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObsidianVaultEntry {
    pub path: String,
    pub ts: Option<u64>,
    pub open: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct DiscoveredVault {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub last_opened: u64,
    pub is_open: bool,
}

/// Discovers all Obsidian vaults registered in the system Obsidian config (~/.config/obsidian/obsidian.json)
pub fn discover_obsidian_vaults() -> Vec<DiscoveredVault> {
    let mut vaults = Vec::new();

    let config_path = dirs::config_dir()
        .map(|p| p.join("obsidian").join("obsidian.json"));

    if let Some(config_path) = config_path {
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(cfg) = serde_json::from_str::<ObsidianConfig>(&content) {
                    for (id, entry) in cfg.vaults {
                        let path = PathBuf::from(&entry.path);
                        if path.exists() {
                            let name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Unnamed Vault")
                                .to_string();

                            vaults.push(DiscoveredVault {
                                id,
                                name,
                                path,
                                last_opened: entry.ts.unwrap_or(0),
                                is_open: entry.open.unwrap_or(false),
                            });
                        }
                    }
                }
            }
        }
    }

    // Sort by most recently used first
    vaults.sort_by(|a, b| b.last_opened.cmp(&a.last_opened));
    vaults
}

/// Resolves the vault to use:
/// 1. Command-line specified name or path
/// 2. Current working directory (if it contains .obsidian)
/// 3. Most recently used registered Obsidian vault
pub fn resolve_vault(target: Option<&str>) -> Result<DiscoveredVault, String> {
    let discovered = discover_obsidian_vaults();

    // 1. If explicit argument given
    if let Some(target) = target {
        // Match by exact or partial name in discovered vaults
        let target_lower = target.to_lowercase();
        if let Some(found) = discovered.iter().find(|v| {
            v.name.to_lowercase() == target_lower || v.name.to_lowercase().contains(&target_lower)
        }) {
            return Ok(found.clone());
        }

        // Match as direct path
        let path = PathBuf::from(target);
        if path.exists() && path.is_dir() {
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Custom Vault")
                .to_string();
            return Ok(DiscoveredVault {
                id: "custom".to_string(),
                name,
                path,
                last_opened: 0,
                is_open: true,
            });
        }

        return Err(format!("Could not find vault matching '{}'", target));
    }

    // 2. Check current directory
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.join(".obsidian").exists() {
        let name = cwd.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Current Vault")
            .to_string();
        return Ok(DiscoveredVault {
            id: "cwd".to_string(),
            name,
            path: cwd,
            last_opened: 0,
            is_open: true,
        });
    }

    // 3. Fallback to most recently opened discovered vault
    if let Some(first) = discovered.first() {
        return Ok(first.clone());
    }

    // 4. Fallback to Documents/punkeahi_obsidian or obsidiankitty if config didn't have it
    if let Some(home) = dirs::home_dir() {
        let punkeahi = home.join("Documents").join("punkeahi_obsidian");
        if punkeahi.exists() {
            return Ok(DiscoveredVault {
                id: "punkeahi".to_string(),
                name: "punkeahi_obsidian".to_string(),
                path: punkeahi,
                last_opened: 0,
                is_open: true,
            });
        }
        let kitty = home.join("Documents").join("obsidiankitty");
        if kitty.exists() {
            return Ok(DiscoveredVault {
                id: "kitty".to_string(),
                name: "obsidiankitty".to_string(),
                path: kitty,
                last_opened: 0,
                is_open: true,
            });
        }
    }

    Err("No Obsidian vaults found on this device. Specify a directory with -v <path>".to_string())
}
