//! Virtual monitor lifecycle management.
//!
//! This module handles creating and destroying virtual monitors in response
//! to commands from the app. In the actual driver build (Windows + WDK),
//! this integrates with IddCx APIs. The logic here is platform-agnostic
//! for testability.

use monitor_splitter_common::{
    PhysicalMonitor, SplitConfig, SplitRegion, VirtualMonitor, MAX_VIRTUAL_MONITORS,
};

/// Manages the lifecycle of virtual monitors.
pub struct MonitorManager {
    /// Currently active virtual monitors.
    virtual_monitors: Vec<VirtualMonitor>,
    /// Known physical monitors.
    physical_monitors: Vec<PhysicalMonitor>,
    /// Counter for generating unique IDs.
    next_id: u32,
}

impl MonitorManager {
    pub fn new() -> Self {
        Self {
            virtual_monitors: Vec::new(),
            physical_monitors: Vec::new(),
            next_id: 0,
        }
    }

    /// Register a physical monitor (called during enumeration).
    pub fn register_physical_monitor(&mut self, monitor: PhysicalMonitor) {
        self.physical_monitors.push(monitor);
    }

    /// Get all registered physical monitors.
    pub fn physical_monitors(&self) -> &[PhysicalMonitor] {
        &self.physical_monitors
    }

    /// Get all active virtual monitors.
    pub fn virtual_monitors(&self) -> &[VirtualMonitor] {
        &self.virtual_monitors
    }

    /// Apply a split configuration, creating virtual monitors.
    ///
    /// This will remove any existing virtual monitors for the target physical
    /// monitor before creating new ones.
    pub fn apply_split(&mut self, config: &SplitConfig) -> Result<Vec<VirtualMonitor>, String> {
        // Validate
        let physical = self
            .physical_monitors
            .iter()
            .find(|m| m.index == config.monitor_index)
            .ok_or_else(|| format!("Physical monitor {} not found", config.monitor_index))?
            .clone();

        if config.regions.is_empty() {
            return Err("No regions specified".to_string());
        }

        if self.virtual_monitors.len() + config.regions.len() > MAX_VIRTUAL_MONITORS {
            return Err(format!(
                "Would exceed maximum of {} virtual monitors",
                MAX_VIRTUAL_MONITORS
            ));
        }

        // Validate regions
        super::edid::validate_regions(&config.regions)?;

        // Remove existing splits for this monitor
        self.virtual_monitors
            .retain(|vm| vm.physical_monitor_index != config.monitor_index);

        // Create new virtual monitors
        let mut created = Vec::new();
        for region in &config.regions {
            let vm = self.create_virtual_monitor(&physical, region);
            created.push(vm.clone());
            self.virtual_monitors.push(vm);
        }

        Ok(created)
    }

    /// Remove all virtual monitors for a specific physical monitor.
    pub fn remove_splits(&mut self, monitor_index: u32) -> usize {
        let before = self.virtual_monitors.len();
        self.virtual_monitors
            .retain(|vm| vm.physical_monitor_index != monitor_index);
        before - self.virtual_monitors.len()
    }

    /// Remove all virtual monitors.
    pub fn remove_all(&mut self) -> usize {
        let count = self.virtual_monitors.len();
        self.virtual_monitors.clear();
        count
    }

    fn create_virtual_monitor(
        &mut self,
        physical: &PhysicalMonitor,
        region: &SplitRegion,
    ) -> VirtualMonitor {
        let id = self.next_id;
        self.next_id += 1;

        let width = (physical.width as f64 * region.width) as u32;
        let height = (physical.height as f64 * region.height) as u32;

        VirtualMonitor {
            id: format!("vm-{}", id),
            physical_monitor_index: physical.index,
            region: region.clone(),
            width,
            height,
        }
    }
}

impl Default for MonitorManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use monitor_splitter_common::equal_horizontal_split;

    fn test_physical_monitor() -> PhysicalMonitor {
        PhysicalMonitor {
            index: 0,
            name: "Test Ultrawide".to_string(),
            width: 3840,
            height: 1080,
            refresh_rate: 60,
        }
    }

    #[test]
    fn test_apply_horizontal_split() {
        let mut mgr = MonitorManager::new();
        mgr.register_physical_monitor(test_physical_monitor());

        let config = equal_horizontal_split(0, 2);
        let result = mgr.apply_split(&config).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].width, 1920);
        assert_eq!(result[0].height, 1080);
        assert_eq!(result[1].width, 1920);
        assert_eq!(result[1].height, 1080);
    }

    #[test]
    fn test_remove_splits() {
        let mut mgr = MonitorManager::new();
        mgr.register_physical_monitor(test_physical_monitor());

        let config = equal_horizontal_split(0, 3);
        mgr.apply_split(&config).unwrap();
        assert_eq!(mgr.virtual_monitors().len(), 3);

        let removed = mgr.remove_splits(0);
        assert_eq!(removed, 3);
        assert_eq!(mgr.virtual_monitors().len(), 0);
    }

    #[test]
    fn test_reapply_replaces() {
        let mut mgr = MonitorManager::new();
        mgr.register_physical_monitor(test_physical_monitor());

        let config2 = equal_horizontal_split(0, 2);
        mgr.apply_split(&config2).unwrap();
        assert_eq!(mgr.virtual_monitors().len(), 2);

        let config3 = equal_horizontal_split(0, 3);
        mgr.apply_split(&config3).unwrap();
        assert_eq!(mgr.virtual_monitors().len(), 3);
    }
}



