use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

pub fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "qdiaps", "qpeek")
}

fn default_eco_mode() -> bool {
    false
}

fn default_show_standalone_warning() -> bool {
    true
}

fn default_vault_path() -> PathBuf {
    UserDirs::new()
        .map(|dirs| dirs.home_dir().join("cheat-sheets"))
        .unwrap_or_else(|| PathBuf::from("~/cheat-sheets"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_eco_mode")]
    pub eco_mode: bool,

    #[serde(default = "default_show_standalone_warning")]
    pub show_standalone_warning: bool,

    #[serde(default = "default_vault_path")]
    pub vault_path: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            eco_mode: default_eco_mode(),
            show_standalone_warning: default_show_standalone_warning(),
            vault_path: default_vault_path(),
        }
    }
}

impl AppConfig {
    pub fn get_path() -> PathBuf {
        get_project_dirs()
            .map(|dirs| dirs.config_dir().join("config.json"))
            .expect("Could not determine config directory")
    }

    pub fn load_or_create() -> Self {
        let path = Self::get_path();

        if !path.exists() {
            info!(target: "fs", "Config file not found. Creating default at: {:?}", path);
            let config = Self::default();
            config.save().unwrap_or_else(|e| {
                warn!(target: "fs", "Failed to save default config: {}", e);
            });
            return config;
        }

        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_else(|e| {
            warn!(target: "fs", "Failed to parse config file: {}. Using defaults.", e);
            Self::default()
        })
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::get_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}
