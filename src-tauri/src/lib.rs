pub mod cli;
pub mod config;
pub mod logging;

use cli::AppCli;
use config::AppConfig;
use std::sync::{Arc, RwLock};
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logging::setup_logger();

    let args = AppCli::parse_args();
    let is_daemon = args.daemon;

    let mut app_config = AppConfig::load_or_create(args.config.map(std::path::PathBuf::from));

    if let Some(custom_vault) = args.vault {
        tracing::info!(target: "daemon", "Overriding vault path from CLI: {}", custom_vault);
        app_config.vault_path = std::path::PathBuf::from(custom_vault);
    }

    if is_daemon {
        tracing::info!(target: "daemon", "Bootstrapping qpeek in Background/Daemon mode...");
    } else {
        tracing::info!(target: "ui", "Bootstrapping qpeek in Standalone/Client mode...");
    }

    let config_state = Arc::new(RwLock::new(app_config));

    let config_event = Arc::clone(&config_state);
    let config_setup = Arc::clone(&config_state);

    let is_daemon_event = is_daemon;
    let is_daemon_run = is_daemon;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            tracing::info!(target: "ipc", "Received IPC signal. Args: {:?}", argv);

            if let Some(window) = app.get_webview_window("main") {
                window.show().unwrap();
                window.set_focus().unwrap();
                tracing::info!(target: "ui", "Window invoked via IPC");
            } else {
                tracing::info!(target: "ui", "Re-creating window (Eco mode active)");
                let new_window = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                    .title("qpeek")
                    .inner_size(800.0, 600.0)
                    .build()
                    .unwrap();

                new_window.set_focus().unwrap();
            }
        }))
        .manage(Arc::clone(&config_state))
        .on_window_event(move |window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() != "main" { return; }

                if !is_daemon_event {
                    tracing::info!(target: "ui", "Standalone app window closed. Exiting process.");
                    return;
                }

                let eco_mode = config_event.read().map(|c| c.eco_mode).unwrap_or(false);

                if !eco_mode {
                    api.prevent_close();
                    window.hide().unwrap();
                    tracing::info!(target: "ui", "Window hidden (Fast mode active)");
                } else {
                    tracing::info!(target: "ui", "Window closing (Eco mode active)");
                }
            }
        })
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();
            let config = config_setup.read().unwrap();

            if is_daemon {
                tracing::info!(target: "daemon", "Daemon is ready. Window spawned but hidden.");
            } else {
                window.show().unwrap();
                window.set_focus().unwrap();

                if config.show_standalone_warning {
                    tracing::warn!(target: "ui", "Standalone mode detected. Broadcasting warning to Vue.");
                    let _ = app.emit("standalone-warning", "Running without daemon may impact performance");
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(move |_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                if is_daemon_run {
                    api.prevent_exit();
                    tracing::info!(target: "daemon", "Exit requested but prevented. Daemon stays in background.");
                } else {
                    tracing::info!(target: "ui", "App exit requested (Standalone mode). Terminating.");
                }
            }
        });
}
