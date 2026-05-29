//! Shared types and protocol definitions for the monitor-splitter project.
//!
//! This crate defines the communication protocol between the Tauri app and
//! the IddCx virtual display driver via named pipes.

use serde::{Deserialize, Serialize};

/// Named pipe path for app ↔ driver communication.
pub const PIPE_NAME: &str = r"\\.\pipe\MonitorSplitter";

/// Maximum number of virtual monitors supported.
pub const MAX_VIRTUAL_MONITORS: usize = 16;

// ─── Split Configuration ───────────────────────────────────────────────────────

/// Orientation of a monitor split.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// A split region defined as a fraction of the physical monitor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitRegion {
    /// Left edge as fraction of total width (0.0–1.0).
    pub x: f64,
    /// Top edge as fraction of total height (0.0–1.0).
    pub y: f64,
    /// Width as fraction of total width (0.0–1.0).
    pub width: f64,
    /// Height as fraction of total height (0.0–1.0).
    pub height: f64,
}

/// A complete split configuration for one physical monitor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitConfig {
    /// Index of the physical monitor to split.
    pub monitor_index: u32,
    /// Regions that define the virtual monitors.
    pub regions: Vec<SplitRegion>,
    /// Human-readable preset name (optional).
    pub preset_name: Option<String>,
}

/// Physical monitor information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalMonitor {
    /// System-assigned monitor index.
    pub index: u32,
    /// Display name / model.
    pub name: String,
    /// Native horizontal resolution.
    pub width: u32,
    /// Native vertical resolution.
    pub height: u32,
    /// Refresh rate in Hz.
    pub refresh_rate: u32,
}

/// Virtual monitor created by the driver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMonitor {
    /// Unique ID assigned by the driver.
    pub id: String,
    /// Which physical monitor this is part of.
    pub physical_monitor_index: u32,
    /// Region on the physical monitor.
    pub region: SplitRegion,
    /// Resolved pixel width.
    pub width: u32,
    /// Resolved pixel height.
    pub height: u32,
}

// ─── Protocol Messages ─────────────────────────────────────────────────────────

/// Messages sent from the app to the driver.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AppToDriver {
    /// Query available physical monitors.
    QueryMonitors,
    /// Apply a split configuration.
    ApplySplit(SplitConfig),
    /// Remove all virtual monitors for a physical monitor.
    RemoveSplits { monitor_index: u32 },
    /// Remove all virtual monitors.
    RemoveAll,
    /// Ping / health check.
    Ping,
}

/// Messages sent from the driver to the app.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DriverToApp {
    /// List of physical monitors.
    MonitorList {
        monitors: Vec<PhysicalMonitor>,
    },
    /// Current virtual monitor state.
    SplitState {
        virtual_monitors: Vec<VirtualMonitor>,
    },
    /// Operation succeeded.
    Ok,
    /// Error response.
    Error {
        message: String,
    },
    /// Pong response.
    Pong,
}

// ─── Preset / Hotkey Config ────────────────────────────────────────────────────

/// A saved preset combining a split config with an optional hotkey binding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// Unique preset name.
    pub name: String,
    /// The split configuration.
    pub config: SplitConfig,
    /// Hotkey binding (e.g., "Ctrl+Alt+1").
    pub hotkey: Option<String>,
}

/// App configuration stored on disk.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// Saved presets.
    pub presets: Vec<Preset>,
    /// Whether to start with Windows.
    pub start_on_boot: bool,
    /// Whether to minimize to system tray.
    pub minimize_to_tray: bool,
}

// ─── Helper Functions ──────────────────────────────────────────────────────────

/// Create a simple N-way equal horizontal split config.
pub fn equal_horizontal_split(monitor_index: u32, n: u32) -> SplitConfig {
    let width = 1.0 / n as f64;
    let regions = (0..n)
        .map(|i| SplitRegion {
            x: i as f64 * width,
            y: 0.0,
            width,
            height: 1.0,
        })
        .collect();

    SplitConfig {
        monitor_index,
        regions,
        preset_name: Some(format!("{}× Horizontal", n)),
    }
}

/// Create a simple N-way equal vertical split config.
pub fn equal_vertical_split(monitor_index: u32, n: u32) -> SplitConfig {
    let height = 1.0 / n as f64;
    let regions = (0..n)
        .map(|i| SplitRegion {
            x: 0.0,
            y: i as f64 * height,
            width: 1.0,
            height,
        })
        .collect();

    SplitConfig {
        monitor_index,
        regions,
        preset_name: Some(format!("{}× Vertical", n)),
    }
}

/// Generate a synthetic EDID for a virtual monitor with given resolution.
///
/// This produces a minimal 128-byte EDID block that Windows will accept.
pub fn generate_synthetic_edid(width: u32, height: u32, monitor_name: &str) -> [u8; 128] {
    let mut edid = [0u8; 128];

    // EDID header
    edid[0..8].copy_from_slice(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]);

    // Manufacturer ID: "VSP" (Virtual Splitter)
    // 'V' = 22, 'S' = 19, 'P' = 16
    // Compressed: ((22 & 0x1F) << 10) | ((19 & 0x1F) << 5) | (16 & 0x1F)
    let mfg: u16 = (22 << 10) | (19 << 5) | 16;
    edid[8] = (mfg >> 8) as u8;
    edid[9] = (mfg & 0xFF) as u8;

    // Product code
    edid[10] = 0x01;
    edid[11] = 0x00;

    // Serial number
    edid[12..16].copy_from_slice(&[0x01, 0x00, 0x00, 0x00]);

    // Week and year of manufacture (week 1, 2024)
    edid[16] = 1;
    edid[17] = 34; // 2024 - 1990

    // EDID version 1.4
    edid[18] = 1;
    edid[19] = 4;

    // Basic display parameters (digital input, 8 bits per color)
    edid[20] = 0xA5;

    // Screen size in cm (approximate)
    edid[21] = (width / 40) as u8; // Horizontal
    edid[22] = (height / 40) as u8; // Vertical

    // Gamma (2.2 = value 120)
    edid[23] = 120;

    // Supported features
    edid[24] = 0x06;

    // Chromaticity coordinates (sRGB)
    edid[25..35].copy_from_slice(&[
        0xEE, 0x91, 0xA3, 0x54, 0x4C, 0x99, 0x26, 0x0F, 0x50, 0x54,
    ]);

    // Established timings
    edid[35] = 0x00;
    edid[36] = 0x00;
    edid[37] = 0x00;

    // Standard timings (unused)
    for i in 38..54 {
        edid[i] = 0x01;
    }

    // Preferred timing descriptor (DTD)
    // Pixel clock = width * height * 60 / 10000
    let pixel_clock = (width * height * 60 / 10000) as u16;
    edid[54] = (pixel_clock & 0xFF) as u8;
    edid[55] = (pixel_clock >> 8) as u8;

    // Horizontal active pixels
    let h_active = width as u16;
    edid[56] = (h_active & 0xFF) as u8;
    // Horizontal blanking
    let h_blank: u16 = 160;
    edid[57] = (h_blank & 0xFF) as u8;
    edid[58] = (((h_active >> 4) & 0xF0) | ((h_blank >> 8) & 0x0F)) as u8;

    // Vertical active lines
    let v_active = height as u16;
    edid[59] = (v_active & 0xFF) as u8;
    // Vertical blanking
    let v_blank: u16 = 35;
    edid[60] = (v_blank & 0xFF) as u8;
    edid[61] = (((v_active >> 4) & 0xF0) | ((v_blank >> 8) & 0x0F)) as u8;

    // Monitor name descriptor (at byte 72)
    edid[72] = 0x00;
    edid[73] = 0x00;
    edid[74] = 0x00;
    edid[75] = 0xFC; // Monitor name tag
    edid[76] = 0x00;
    let name_bytes = monitor_name.as_bytes();
    let len = name_bytes.len().min(13);
    edid[77..77 + len].copy_from_slice(&name_bytes[..len]);
    if len < 13 {
        edid[77 + len] = 0x0A; // Line feed terminator
    }

    // Calculate checksum
    let sum: u8 = edid[0..127].iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    edid[127] = 0u8.wrapping_sub(sum);

    edid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_split() {
        let config = equal_horizontal_split(0, 2);
        assert_eq!(config.regions.len(), 2);
        assert!((config.regions[0].width - 0.5).abs() < f64::EPSILON);
        assert!((config.regions[1].x - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_vertical_split() {
        let config = equal_vertical_split(0, 3);
        assert_eq!(config.regions.len(), 3);
        assert!((config.regions[0].height - 1.0 / 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_edid_checksum() {
        let edid = generate_synthetic_edid(1920, 1080, "VSplit-1");
        let sum: u8 = edid.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let msg = AppToDriver::ApplySplit(equal_horizontal_split(0, 2));
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: AppToDriver = serde_json::from_str(&json).unwrap();
        match parsed {
            AppToDriver::ApplySplit(config) => {
                assert_eq!(config.regions.len(), 2);
            }
            _ => panic!("Wrong variant"),
        }
    }
}

