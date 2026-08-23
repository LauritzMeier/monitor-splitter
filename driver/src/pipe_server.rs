//! Named pipe server for app ↔ driver communication.
//!
//! The driver hosts a named pipe server that the Tauri app connects to.
//! Messages are JSON-encoded, one per line (newline-delimited JSON).
//!
//! In the actual driver context, this runs in a system thread created by
//! the driver's DriverEntry. For testing and the user-mode service fallback,
//! it runs as a normal async task.

use monitor_splitter_common::{AppToDriver, DriverToApp, PIPE_NAME};

/// Configuration for the pipe server.
pub struct PipeServerConfig {
    /// Named pipe path.
    pub pipe_name: String,
    /// Maximum concurrent connections.
    pub max_connections: u32,
}

impl Default for PipeServerConfig {
    fn default() -> Self {
        Self {
            pipe_name: PIPE_NAME.to_string(),
            max_connections: 4,
        }
    }
}

/// Process an incoming message and produce a response.
///
/// This is the core message handler, decoupled from transport for testability.
pub fn handle_message(
    msg: &AppToDriver,
    manager: &mut super::monitor_manager::MonitorManager,
) -> DriverToApp {
    match msg {
        AppToDriver::Ping => DriverToApp::Pong,

        AppToDriver::QueryMonitors => DriverToApp::MonitorList {
            monitors: manager.physical_monitors().to_vec(),
        },

        AppToDriver::QuerySplitState => DriverToApp::SplitState {
            virtual_monitors: manager.virtual_monitors().to_vec(),
        },

        AppToDriver::ApplySplit(config) => match manager.apply_split(config) {
            Ok(_) => DriverToApp::SplitState {
                virtual_monitors: manager.virtual_monitors().to_vec(),
            },
            Err(e) => DriverToApp::Error { message: e },
        },

        AppToDriver::RemoveSplits { monitor_index } => {
            manager.remove_splits(*monitor_index);
            DriverToApp::SplitState {
                virtual_monitors: manager.virtual_monitors().to_vec(),
            }
        }

        AppToDriver::RemoveAll => {
            manager.remove_all();
            DriverToApp::Ok
        }
    }
}

/// Serialize a message for transmission over the pipe.
pub fn serialize_message<T: serde::Serialize>(msg: &T) -> Result<Vec<u8>, serde_json::Error> {
    let mut bytes = serde_json::to_vec(msg)?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// Deserialize a message received from the pipe.
pub fn deserialize_message<'a, T: serde::Deserialize<'a>>(data: &'a [u8]) -> Result<T, serde_json::Error> {
    serde_json::from_slice(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use monitor_splitter_common::{equal_horizontal_split, PhysicalMonitor};

    fn setup_manager() -> super::super::monitor_manager::MonitorManager {
        let mut mgr = super::super::monitor_manager::MonitorManager::new();
        mgr.register_physical_monitor(PhysicalMonitor {
            index: 0,
            name: "Test Monitor".to_string(),
            width: 3840,
            height: 1080,
            refresh_rate: 60,
        });
        mgr
    }

    #[test]
    fn test_ping_pong() {
        let mut mgr = setup_manager();
        let resp = handle_message(&AppToDriver::Ping, &mut mgr);
        assert!(matches!(resp, DriverToApp::Pong));
    }

    #[test]
    fn test_query_monitors() {
        let mut mgr = setup_manager();
        let resp = handle_message(&AppToDriver::QueryMonitors, &mut mgr);
        match resp {
            DriverToApp::MonitorList { monitors } => {
                assert_eq!(monitors.len(), 1);
                assert_eq!(monitors[0].width, 3840);
            }
            _ => panic!("Expected MonitorList"),
        }
    }

    #[test]
    fn test_apply_split_via_message() {
        let mut mgr = setup_manager();
        let config = equal_horizontal_split(0, 2);
        let resp = handle_message(&AppToDriver::ApplySplit(config), &mut mgr);
        match resp {
            DriverToApp::SplitState { virtual_monitors } => {
                assert_eq!(virtual_monitors.len(), 2);
            }
            _ => panic!("Expected SplitState"),
        }
    }

    #[test]
    fn test_message_serialization_roundtrip() {
        let msg = AppToDriver::Ping;
        let bytes = serialize_message(&msg).unwrap();
        let parsed: AppToDriver = deserialize_message(&bytes[..bytes.len() - 1]).unwrap();
        assert!(matches!(parsed, AppToDriver::Ping));
    }
}



