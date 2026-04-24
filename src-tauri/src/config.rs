use directories::{ProjectDirs, UserDirs};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tracing::{error, info, warn};

pub fn get_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "qpeek", "qpeek")
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

    pub fn load_or_create(path: &PathBuf) -> Self {
        if !path.exists() {
            info!(target: "fs", "Config file not found. Creating default at: {:?}", path);
            let config = Self::default();
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let json = serde_json::to_string_pretty(&config).unwrap_or_default();
            let _ = fs::write(path, json);
            return config;
        }

        let content = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_else(|e| {
            warn!(target: "fs", "Failed to parse config file at {:?}: {}. Using defaults.", path, e);
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

pub fn spawn_watcher(app: AppHandle, state: Arc<RwLock<AppConfig>>, path: PathBuf) {
    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut debouncer = new_debouncer(Duration::from_millis(300), tx).unwrap();

        if let Some(parent) = path.parent() {
            debouncer
                .watcher()
                .watch(parent, RecursiveMode::NonRecursive)
                .unwrap();
        }

        for res in rx {
            match res {
                Ok(events) => {
                    let mut is_target_changed = false;
                    for event in events {
                        if event.path == path {
                            is_target_changed = true;
                            break;
                        }
                    }

                    if is_target_changed {
                        let content = fs::read_to_string(&path).unwrap_or_default();
                        if let Ok(new_config) = serde_json::from_str::<AppConfig>(&content) {
                            info!(target: "fs", "Config file changed, hot-reloading...");

                            if let Ok(mut lock) = state.write() {
                                *lock = new_config.clone();
                            }

                            let _ = app.emit("config-hot-reload", new_config);
                        }
                    }
                }
                Err(e) => error!(target: "fs", "Watcher error: {:?}", e),
            }
        }
    });
}
