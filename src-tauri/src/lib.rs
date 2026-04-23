pub mod cli;
pub mod logging;

use cli::AppCli;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logging::setup_logger();

    let args = AppCli::parse_args();

    if args.daemon {
        tracing::info!(target: "daemon", "Bootstrapping qpeek in Background/Daemon mode...");
    } else {
        tracing::info!(target: "ui", "Bootstrapping qpeek in Standalone mode...")
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {
            tracing::info!(
                target: "ipc",
                "Received IPC signal from secondary instance. Args: {:?}",
                argv
            );
        }))
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
