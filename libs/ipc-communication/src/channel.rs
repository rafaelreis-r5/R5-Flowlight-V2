// Cross-platform IPC Channel Implementation

use crate::{IPCMessage, IPCResult, IPCError, IPC_PIPE_NAME};
use tokio::sync::mpsc;
use std::sync::Arc;

pub type IPCSender = mpsc::UnboundedSender<IPCMessage>;
pub type IPCReceiver = mpsc::UnboundedReceiver<IPCMessage>;

// Platform-specific IPC channel creation
#[cfg(unix)]
pub mod unix {
    use super::*;
    use tokio::net::{UnixListener, UnixStream};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use std::path::Path;
    
    pub const SOCKET_PATH: &str = "/tmp/r5_flowlight.sock";
    
    pub async fn create_ipc_server() -> IPCResult<(IPCSender, IPCReceiver)> {
        // Remove existing socket if it exists
        if Path::new(SOCKET_PATH).exists() {
            std::fs::remove_file(SOCKET_PATH)?;
        }
        
        let listener = UnixListener::bind(SOCKET_PATH)
            .map_err(|e| IPCError::ConnectionFailed(e.to_string()))?;
        
        let (tx, rx) = mpsc::unbounded_channel();
        
        // Spawn server task
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            log::info!("IPC server listening on {}", SOCKET_PATH);
            
            while let Ok((stream, _)) = listener.accept().await {
                let tx = tx_clone.clone();
                tokio::spawn(handle_client_connection(stream, tx));
            }
        });
        
        Ok((tx, rx))
    }
    
    pub async fn create_ipc_client() -> IPCResult<(IPCSender, IPCReceiver)> {
        let stream = UnixStream::connect(SOCKET_PATH).await
            .map_err(|e| IPCError::ConnectionFailed(e.to_string()))?;
            
        let (tx, rx) = mpsc::unbounded_channel();
        
        // Handle bidirectional communication
        tokio::spawn(handle_client_stream(stream, tx.clone(), rx));
        
        Ok((tx, rx))
    }
    
    async fn handle_client_connection(mut stream: UnixStream, tx: IPCSender) {
        let mut buffer = vec![0; 8192];
        
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => break, // Connection closed
                Ok(n) => {
                    if let Ok(message_str) = std::str::from_utf8(&buffer[..n]) {
                        if let Ok(message) = serde_json::from_str::<IPCMessage>(message_str) {
                            if tx.send(message).is_err() {
                                break;
                            }
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }
    
    async fn handle_client_stream(mut stream: UnixStream, tx: IPCSender, mut rx: IPCReceiver) {
        let (mut reader, mut writer) = stream.split();
        
        // Read task
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let mut buffer = vec![0; 8192];
            
            while let Ok(n) = reader.read(&mut buffer).await {
                if n == 0 { break; }
                
                if let Ok(message_str) = std::str::from_utf8(&buffer[..n]) {
                    if let Ok(message) = serde_json::from_str::<IPCMessage>(message_str) {
                        let _ = tx_clone.send(message);
                    }
                }
            }
        });
        
        // Write task
        while let Some(message) = rx.recv().await {
            if let Ok(message_str) = serde_json::to_string(&message) {
                if writer.write_all(message_str.as_bytes()).await.is_err() {
                    break;
                }
            }
        }
    }
}

#[cfg(windows)]
pub mod windows {
    use super::*;
    use windows::Win32::System::Pipes::*;
    use windows::Win32::Foundation::*;
    use windows::core::*;
    
    pub const PIPE_NAME: &str = r"\\.\pipe\r5_flowlight_ipc";
    
    pub async fn create_ipc_server() -> IPCResult<(IPCSender, IPCReceiver)> {
        let (tx, rx) = mpsc::unbounded_channel();
        
        // Create named pipe server
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            log::info!("IPC server listening on {}", PIPE_NAME);
            
            loop {
                unsafe {
                    let pipe_handle = CreateNamedPipeA(
                        PCSTR(PIPE_NAME.as_ptr()),
                        PIPE_ACCESS_DUPLEX,
                        PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                        PIPE_UNLIMITED_INSTANCES,
                        8192,
                        8192,
                        0,
                        None,
                    );
                    
                    if pipe_handle != INVALID_HANDLE_VALUE {
                        let tx = tx_clone.clone();
                        tokio::spawn(handle_named_pipe(pipe_handle, tx));
                    }
                }
            }
        });
        
        Ok((tx, rx))
    }
    
    pub async fn create_ipc_client() -> IPCResult<(IPCSender, IPCReceiver)> {
        // Connect to named pipe
        // Implementation for Windows client connection
        todo!("Implement Windows IPC client")
    }
    
    async fn handle_named_pipe(pipe_handle: HANDLE, tx: IPCSender) {
        // Implementation for handling Windows named pipe communication
        todo!("Implement Windows named pipe handler")
    }
}

// Export platform-specific functions
#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
pub use windows::*;