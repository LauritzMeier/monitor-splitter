//! Tauri command handlers exposed to the frontend.

use monitor_splitter_common::*;

#[tauri::command]
pub async fn get_physical_monitors() -> Result<Vec<PhysicalMonitor>, String> {
    // TODO: Query from driver via named pipe
    // For now, return mock data for UI development
    Ok(vec![PhysicalMonitor {
        index: 0,
        name: "Primary Monitor".to_string(),
        width: 3840,
        height: 1080,
        refresh_rate: 60,
    }])
}

#[tauri::command]
pub async fn get_virtual_monitors() -> Result<Vec<VirtualMonitor>, String> {
    // TODO: Query from driver
    Ok(vec![])
}

#[tauri::command]
pub async fn apply_split(config: SplitConfig) -> Result<Vec<VirtualMonitor>, String> {
    // TODO: Send to driver via named pipe
    tracing::info!("Applying split: {:?}", config);

    // For now, compute virtual monitors locally
    let physical_width = 3840u32; // TODO: look up from actual monitor
    let physical_height = 1080u32;

    let vms: Vec<VirtualMonitor> = config
        .regions
        .iter()
        .enumerate()
        .map(|(i, region)| VirtualMonitor {
            id: format!("vm-{}", i),
            physical_monitor_index: config.monitor_index,
            region: region.clone(),
            width: (physical_width as f64 * region.width) as u32,
            height: (physical_height as f64 * region.height) as u32,
        })
        .collect();

    Ok(vms)
}

#[tauri::command]
pub async fn remove_splits(monitor_index: u32) -> Result<(), String> {
    tracing::info!("Removing splits for monitor {}", monitor_index);
    // TODO: Send to driver
    Ok(())
}

#[tauri::command]
pub async fn remove_all() -> Result<(), String> {
    tracing::info!("Removing all virtual monitors");
    // TODO: Send to driver
    Ok(())
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


