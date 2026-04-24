use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
