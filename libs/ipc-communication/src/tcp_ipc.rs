// TCP-based IPC for real cross-process communication
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json;
use crate::{IPCMessage, IPCResult, IPCError};

const IPC_PORT: u16 = 19755;

pub struct TcpIPCServer {
    listener: Option<TcpListener>,
    clients: Arc<RwLock<Vec<TcpStream>>>,
}

impl TcpIPCServer {
    pub async fn new() -> IPCResult<Self> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", IPC_PORT)).await
            .map_err(|e| IPCError::ConnectionFailed(format!("Failed to bind TCP server: {}", e)))?;
        
        println!("🔌 TCP IPC Server listening on port {}", IPC_PORT);
        
        Ok(Self {
            listener: Some(listener),
            clients: Arc::new(RwLock::new(Vec::new())),
        })
    }
    
    pub async fn start(&mut self) -> IPCResult<()> {
        if let Some(listener) = self.listener.take() {
            let clients = self.clients.clone();
            
            tokio::spawn(async move {
                loop {
                    match listener.accept().await {
                        Ok((stream, addr)) => {
                            println!("🔌 New client connected: {}", addr);
                            let mut clients_lock = clients.write().await;
                            clients_lock.push(stream);
                        }
                        Err(e) => {
                            println!("❌ Failed to accept connection: {}", e);
                        }
                    }
                }
            });
        }
        
        Ok(())
    }
    
    pub async fn broadcast(&self, message: IPCMessage) -> IPCResult<()> {
        let message_json = serde_json::to_string(&message)
            .map_err(|e| IPCError::SerializationError(e))?;
        
        let mut clients_lock = self.clients.write().await;
        let mut to_remove = Vec::new();
        
        for (i, stream) in clients_lock.iter_mut().enumerate() {
            match stream.write_all(format!("{}\n", message_json).as_bytes()).await {
                Ok(_) => {
                    println!("📤 Message sent to client {}", i);
                }
                Err(_) => {
                    println!("❌ Failed to send to client {}, marking for removal", i);
                    to_remove.push(i);
                }
            }
        }
        
        // Remove disconnected clients
        for &i in to_remove.iter().rev() {
            clients_lock.remove(i);
        }
        
        Ok(())
    }
}

pub struct TcpIPCClient {
    stream: Option<TcpStream>,
}

impl TcpIPCClient {
    pub async fn new() -> IPCResult<Self> {
        let stream = TcpStream::connect(format!("127.0.0.1:{}", IPC_PORT)).await
            .map_err(|e| IPCError::ConnectionFailed(format!("Failed to connect to TCP server: {}", e)))?;
        
        println!("🔌 TCP IPC Client connected");
        
        Ok(Self {
            stream: Some(stream),
        })
    }
    
    pub async fn send(&mut self, message: IPCMessage) -> IPCResult<()> {
        if let Some(stream) = &mut self.stream {
            let message_json = serde_json::to_string(&message)
                .map_err(|e| IPCError::SerializationError(e))?;
            
            stream.write_all(format!("{}\n", message_json).as_bytes()).await
                .map_err(|e| IPCError::SendFailed(format!("Failed to send message: {}", e)))?;
            
            println!("📤 TCP Client sent: {:?}", message);
            Ok(())
        } else {
            Err(IPCError::SendFailed("No connection available".to_string()))
        }
    }
    
    pub async fn receive(&mut self) -> IPCResult<IPCMessage> {
        if let Some(stream) = &mut self.stream {
            let mut buffer = vec![0; 1024];
            let n = stream.read(&mut buffer).await
                .map_err(|e| IPCError::ReceiveFailed(format!("Failed to read from stream: {}", e)))?;
            
            if n == 0 {
                return Err(IPCError::ReceiveFailed("Connection closed".to_string()));
            }
            
            let message_str = String::from_utf8_lossy(&buffer[..n]);
            let message_str = message_str.trim();
            
            let message: IPCMessage = serde_json::from_str(message_str)
                .map_err(|e| IPCError::SerializationError(e))?;
            
            println!("📨 TCP Client received: {:?}", message);
            Ok(message)
        } else {
            Err(IPCError::ReceiveFailed("No connection available".to_string()))
        }
    }
}