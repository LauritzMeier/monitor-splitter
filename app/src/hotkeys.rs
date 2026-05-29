//! Global hotkey management.
//!
//! Registers system-wide hotkeys that trigger preset activation,
//! allowing users to switch monitor layouts instantly.

use monitor_splitter_common::Preset;

/// Manages global hotkey registrations.
pub struct HotkeyManager {
    // In production, this holds global_hotkey::GlobalHotKeyManager
    // and maps hotkey IDs to preset names.
    registered: Vec<(String, String)>, // (hotkey_str, preset_name)
}

impl HotkeyManager {
    pub fn new() -> Self {
        Self {
            registered: Vec::new(),
        }
    }

    /// Register hotkeys for all presets that have bindings.
    pub fn register_presets(&mut self, presets: &[Preset]) -> anyhow::Result<()> {
        self.unregister_all();

        for preset in presets {
            if let Some(hotkey) = &preset.hotkey {
                tracing::info!("Registering hotkey '{}' for preset '{}'", hotkey, preset.name);
                self.registered.push((hotkey.clone(), preset.name.clone()));
                // TODO: Actually register with global_hotkey crate
                // let hk = hotkey.parse::<HotKey>()?;
                // self.manager.register(hk)?;
            }
        }

        Ok(())
    }

    /// Unregister all hotkeys.
    pub fn unregister_all(&mut self) {
        self.registered.clear();
        // TODO: Unregister from global_hotkey
    }

    /// Look up which preset a hotkey ID maps to.
    pub fn preset_for_hotkey(&self, hotkey_str: &str) -> Option<&str> {
        self.registered
            .iter()
            .find(|(hk, _)| hk == hotkey_str)
            .map(|(_, name)| name.as_str())
    }
}

