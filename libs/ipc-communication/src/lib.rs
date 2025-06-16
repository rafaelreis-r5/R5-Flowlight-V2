// IPC Communication Library - Cross-Platform
// Handles communication between main-app, search-daemon, and search-overlay

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IPCMessage {
    // Overlay control
    ToggleOverlay,
    ShowOverlay { query: Option<String> },
    HideOverlay,
    
    // Search operations
    SearchQuery { query: String, session_id: String },
    SearchResults { results: Vec<SearchResult>, session_id: String },
    ClearResults,
    
    // Configuration
    UpdateModule { module_id: String },
    GetCurrentModule,
    ModuleChanged { module_id: String },
    
    // System control
    StartDaemon,
    StopDaemon,
    DaemonStatus { running: bool, pid: Option<u32> },
    
    // Handshake
    Ping,
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: Option<String>,
    pub action_type: String,
    pub metadata: HashMap<String, String>,
}

#[derive(thiserror::Error, Debug)]
pub enum IPCError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Send failed: {0}")]
    SendFailed(String),
    #[error("Receive failed: {0}")]
    ReceiveFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type IPCResult<T> = Result<T, IPCError>;

// Cross-platform IPC implementation
// TODO: Fix async lifetime issues
// pub mod channel;
// pub mod server;
// pub mod client;
pub mod simple; // Simplified version for testing
pub mod real_simple; // Real but simple IPC
pub mod tcp_ipc; // TCP-based cross-process IPC

// pub use channel::*;
// pub use server::*;
// pub use client::*;
pub use simple::{test_basic_ipc, MessageHandler};
pub use real_simple::{IPCServer, IPCClient, debug_message_bus};
pub use tcp_ipc::{TcpIPCServer, TcpIPCClient};

// Constants
pub const IPC_PIPE_NAME: &str = "r5_flowlight_ipc";
pub const IPC_TIMEOUT_MS: u64 = 5000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = IPCMessage::SearchQuery {
            query: "test".to_string(),
            session_id: Uuid::new_v4().to_string(),
        };
        
        let serialized = serde_json::to_string(&msg).unwrap();
        let deserialized: IPCMessage = serde_json::from_str(&serialized).unwrap();
        
        match deserialized {
            IPCMessage::SearchQuery { query, .. } => assert_eq!(query, "test"),
            _ => panic!("Deserialization failed"),
        }
    }
}