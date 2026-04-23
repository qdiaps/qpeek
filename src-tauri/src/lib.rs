pub mod cli;
pub mod logging;

use cli::AppCli;
use tauri::{Emitter, Manager, WindowEvent};

struct AppConfig {
    eco_mode: bool,
    show_standalone_warning: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logging::setup_logger();

    let args = AppCli::parse_args();
    let is_daemon = args.daemon;

    let config = AppConfig {
        eco_mode: false, // false = hide window, true = close window
        show_standalone_warning: true,
    };

    if is_daemon {
        tracing::info!(target: "daemon", "Bootstrapping qpeek in Background/Daemon mode...");
    } else {
        tracing::info!(target: "ui", "Bootstrapping qpeek in Standalone/Client mode...")
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            tracing::info!(target: "ipc", "Received IPC signal. Args: {:?}", argv);

            if let Some(window) = app.get_webview_window("main") {
                window.show().unwrap();
                window.set_focus().unwrap();
                tracing::info!(target: "ui", "Window invoked via IPC");
            }
        }))
        .setup(move |app| {
            let window = app.get_webview_window("main").unwrap();

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

            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    if !config.eco_mode {
                        api.prevent_close();
                        window_clone.hide().unwrap();
                        tracing::info!(target: "ui", "Window hidden (Fast mode active)");
                    } else {
                        tracing::info!(target: "ui", "Window closing (Eco mode active)");
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
