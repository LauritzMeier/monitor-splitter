//! IddCx Virtual Display Driver for Monitor Splitter
//!
//! This driver registers virtual display adapters with Windows via the
//! Indirect Display Driver (IddCx) framework. Each virtual monitor maps
//! to a region of a physical monitor's framebuffer, enabling DisplayFusion-style
//! splitting where each sub-monitor renders independently to its region.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │  Windows Display Stack (DWM / WDDM)     │
//! └────────────────┬────────────────────────┘
//!                  │
//! ┌────────────────▼────────────────────────┐
//! │  IddCx Framework                         │
//! │  - Adapter creation                      │
//! │  - Monitor arrival/departure             │
//! │  - Swapchain management                  │
//! └────────────────┬────────────────────────┘
//!                  │
//! ┌────────────────▼────────────────────────┐
//! │  This Driver                             │
//! │  - Named pipe server for app comms       │
//! │  - Synthetic EDID generation             │
//! │  - Framebuffer region mapping            │
//! │  - Dynamic monitor create/destroy        │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## Building
//!
//! This crate requires the Windows Driver Kit (WDK) and must be compiled on Windows.
//! It is typically built via GitHub Actions CI. The wdk crate dependencies are
//! commented out for cross-platform workspace compatibility — uncomment them
//! and the build script for actual driver compilation.

pub mod edid;
pub mod monitor_manager;
pub mod pipe_server;

// Re-export common types
pub use monitor_splitter_common::*;

