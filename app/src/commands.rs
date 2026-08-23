//! Tauri command handlers exposed to the frontend.

use monitor_splitter_common::*;

#[cfg(windows)]
async fn send_driver_message(msg: AppToDriver) -> Result<DriverToApp, String> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::windows::named_pipe::ClientOptions;

    let mut pipe = ClientOptions::new()
        .open(PIPE_NAME)
        .map_err(|e| format!("Failed to connect to driver pipe {}: {}", PIPE_NAME, e))?;

    let payload = serde_json::to_vec(&msg).map_err(|e| format!("Failed to encode request: {}", e))?;
    pipe.write_all(&payload)
        .await
        .map_err(|e| format!("Failed to send request to driver: {}", e))?;
    pipe.write_all(b"\n")
        .await
        .map_err(|e| format!("Failed to finish request: {}", e))?;
    pipe.flush()
        .await
        .map_err(|e| format!("Failed to flush request to driver: {}", e))?;

    let mut reader = BufReader::new(pipe);
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .await
        .map_err(|e| format!("Failed to read driver response: {}", e))?;

    if line.trim().is_empty() {
        return Err("Driver returned an empty response".to_string());
    }

    serde_json::from_str::<DriverToApp>(line.trim())
        .map_err(|e| format!("Failed to decode driver response: {}", e))
}

#[cfg(not(windows))]
async fn send_driver_message(_msg: AppToDriver) -> Result<DriverToApp, String> {
    Err("Monitor splitting is only available on Windows with the driver installed".to_string())
}

#[tauri::command]
pub async fn get_physical_monitors() -> Result<Vec<PhysicalMonitor>, String> {
    match send_driver_message(AppToDriver::QueryMonitors).await? {
        DriverToApp::MonitorList { monitors } => Ok(monitors),
        DriverToApp::Error { message } => Err(message),
        other => Err(format!("Unexpected response from driver: {:?}", other)),
    }
}

#[tauri::command]
pub async fn get_virtual_monitors() -> Result<Vec<VirtualMonitor>, String> {
    match send_driver_message(AppToDriver::QuerySplitState).await? {
        DriverToApp::SplitState { virtual_monitors } => Ok(virtual_monitors),
        DriverToApp::Error { message } => Err(message),
        other => Err(format!("Unexpected response from driver: {:?}", other)),
    }
}

#[tauri::command]
pub async fn apply_split(config: SplitConfig) -> Result<Vec<VirtualMonitor>, String> {
    tracing::info!("Applying split: {:?}", config);

    match send_driver_message(AppToDriver::ApplySplit(config)).await? {
        DriverToApp::SplitState { virtual_monitors } => Ok(virtual_monitors),
        DriverToApp::Error { message } => Err(message),
        other => Err(format!("Unexpected response from driver: {:?}", other)),
    }
}

#[tauri::command]
pub async fn remove_splits(monitor_index: u32) -> Result<(), String> {
    tracing::info!("Removing splits for monitor {}", monitor_index);
    match send_driver_message(AppToDriver::RemoveSplits { monitor_index }).await? {
        DriverToApp::SplitState { .. } | DriverToApp::Ok => Ok(()),
        DriverToApp::Error { message } => Err(message),
        other => Err(format!("Unexpected response from driver: {:?}", other)),
    }
}

#[tauri::command]
pub async fn remove_all() -> Result<(), String> {
    tracing::info!("Removing all virtual monitors");
    match send_driver_message(AppToDriver::RemoveAll).await? {
        DriverToApp::Ok | DriverToApp::SplitState { .. } => Ok(()),
        DriverToApp::Error { message } => Err(message),
        other => Err(format!("Unexpected response from driver: {:?}", other)),
    }
}

#[tauri::command]
pub async fn get_presets() -> Result<Vec<Preset>, String> {
    // TODO: Load from config
    Ok(vec![])
}

#[tauri::command]
pub async fn save_preset(preset: Preset) -> Result<(), String> {
    tracing::info!("Saving preset: {}", preset.name);
    // TODO: Save to config
    Ok(())
}

#[tauri::command]
pub async fn delete_preset(name: String) -> Result<(), String> {
    tracing::info!("Deleting preset: {}", name);
    // TODO: Delete from config
    Ok(())
}

#[tauri::command]
pub async fn get_config() -> Result<AppConfig, String> {
    Ok(AppConfig::default())
}

#[tauri::command]
pub async fn save_config(config: AppConfig) -> Result<(), String> {
    tracing::info!("Saving config with {} preset(s)", config.presets.len());
    // TODO: Persist config
    Ok(())
}




