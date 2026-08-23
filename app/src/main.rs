//! Monitor Splitter — Tauri application entry point.
//!
//! This is the main executable that provides:
//! - System tray icon with quick actions
//! - Web-based UI for monitor layout configuration
//! - Global hotkey registration for preset switching
//! - Named pipe client to communicate with the driver

mod commands;

use tracing_subscriber::EnvFilter;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("monitor_splitter=info".parse().unwrap()))
        .init();

    tracing::info!("Starting Monitor Splitter");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_physical_monitors,
            commands::get_virtual_monitors,
            commands::apply_split,
            commands::remove_splits,
            commands::remove_all,
            commands::get_presets,
            commands::save_preset,
            commands::delete_preset,
            commands::get_config,
            commands::save_config,
        ])
        .setup(|_app| {
            tracing::info!("App setup complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



