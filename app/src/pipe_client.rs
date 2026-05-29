//! Named pipe client for communicating with the driver.
//!
//! Connects to the driver's named pipe server and sends/receives
//! JSON-encoded messages.

use monitor_splitter_common::{AppToDriver, DriverToApp, PIPE_NAME};
use anyhow::Result;

/// Client for communicating with the monitor-splitter driver.
pub struct PipeClient {
    pipe_name: String,
    connected: bool,
}

impl PipeClient {
    pub fn new() -> Self {
        Self {
            pipe_name: PIPE_NAME.to_string(),
            connected: false,
        }
    }

    /// Attempt to connect to the driver's named pipe.
    pub async fn connect(&mut self) -> Result<()> {
        tracing::info!("Connecting to driver pipe: {}", self.pipe_name);

        // TODO: Implement actual Windows named pipe connection
        // On Windows:
        //   use tokio::net::windows::named_pipe::ClientOptions;
        //   let client = ClientOptions::new().open(&self.pipe_name)?;
        //
        // For now, mark as not connected (driver not running)
        self.connected = false;
        tracing::warn!("Driver pipe not available (driver not installed)");

        Ok(())
    }

    /// Send a message to the driver and await response.
    pub async fn send(&self, msg: &AppToDriver) -> Result<DriverToApp> {
        if !self.connected {
            return Ok(DriverToApp::Error {
                message: "Not connected to driver".to_string(),
            });
        }

        let _payload = serde_json::to_vec(msg)?;

        // TODO: Write to pipe, read response
        // let mut buf = vec![0u8; 4096];
        // pipe.write_all(&payload).await?;
        // pipe.write_all(b"\n").await?;
        // let n = pipe.read(&mut buf).await?;
        // let response: DriverToApp = serde_json::from_slice(&buf[..n])?;

        Ok(DriverToApp::Error {
            message: "Pipe communication not yet implemented".to_string(),
        })
    }

    /// Check if connected to the driver.
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

