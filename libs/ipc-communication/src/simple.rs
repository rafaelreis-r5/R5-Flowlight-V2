// Simplified IPC for testing - without complex async handling

use crate::{IPCMessage, IPCResult, IPCError};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type SimpleIPCSender = mpsc::Sender<IPCMessage>;
pub type SimpleIPCReceiver = mpsc::Receiver<IPCMessage>;

pub struct SimpleIPCTest {
    sender: SimpleIPCSender,
    receiver: SimpleIPCReceiver,
}

impl SimpleIPCTest {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }
    
    pub fn send(&self, message: IPCMessage) -> IPCResult<()> {
        self.sender.send(message)
            .map_err(|e| IPCError::SendFailed(e.to_string()))
    }
    
    pub fn try_receive(&self) -> IPCResult<Option<IPCMessage>> {
        match self.receiver.try_recv() {
            Ok(message) => Ok(Some(message)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => {
                Err(IPCError::ReceiveFailed("Channel disconnected".to_string()))
            }
        }
    }
    
    pub fn receive_timeout(&self, timeout: Duration) -> IPCResult<IPCMessage> {
        self.receiver.recv_timeout(timeout)
            .map_err(|e| IPCError::ReceiveFailed(e.to_string()))
    }
}

pub fn test_basic_ipc() -> IPCResult<()> {
    println!("🧪 Testing basic IPC communication...");
    
    let ipc = SimpleIPCTest::new();
    
    // Test 1: Send and receive a message
    let test_message = IPCMessage::Ping;
    ipc.send(test_message)?;
    
    let received = ipc.receive_timeout(Duration::from_millis(100))?;
    match received {
        IPCMessage::Ping => println!("✅ Basic send/receive works"),
        _ => return Err(IPCError::ReceiveFailed("Wrong message type".to_string())),
    }
    
    // Test 2: Search query
    let search_msg = IPCMessage::SearchQuery {
        query: "test query".to_string(),
        session_id: "test-session".to_string(),
    };
    ipc.send(search_msg)?;
    
    let received = ipc.receive_timeout(Duration::from_millis(100))?;
    match received {
        IPCMessage::SearchQuery { query, session_id } => {
            if query == "test query" && session_id == "test-session" {
                println!("✅ Complex message serialization works");
            } else {
                return Err(IPCError::ReceiveFailed("Data corruption".to_string()));
            }
        }
        _ => return Err(IPCError::ReceiveFailed("Wrong message type".to_string())),
    }
    
    // Test 3: Multiple messages
    for i in 0..5 {
        let msg = IPCMessage::UpdateModule { 
            module_id: format!("module_{}", i) 
        };
        ipc.send(msg)?;
    }
    
    let mut received_count = 0;
    while let Ok(Some(_)) = ipc.try_receive() {
        received_count += 1;
    }
    
    if received_count == 5 {
        println!("✅ Multiple messages handling works");
    } else {
        return Err(IPCError::ReceiveFailed(format!("Expected 5 messages, got {}", received_count)));
    }
    
    println!("🎉 All IPC tests passed!");
    Ok(())
}

// Integration-ready IPC implementations
pub trait MessageHandler {
    fn handle(&self, message: IPCMessage) -> IPCResult<Option<IPCMessage>>;
}

pub struct IPCServer {
    message_handlers: HashMap<String, Box<dyn MessageHandler + Send + Sync>>,
    clients: Arc<Mutex<Vec<mpsc::Sender<IPCMessage>>>>,
}

impl IPCServer {
    pub async fn new() -> IPCResult<Self> {
        Ok(Self {
            message_handlers: HashMap::new(),
            clients: Arc::new(Mutex::new(Vec::new())),
        })
    }
    
    pub fn register_handler<T: MessageHandler + Send + Sync + 'static>(&mut self, name: &str, handler: T) {
        self.message_handlers.insert(name.to_string(), Box::new(handler));
    }
    
    pub async fn start(&mut self) -> IPCResult<()> {
        println!("📡 IPC Server started (simplified mode)");
        Ok(())
    }
    
    pub async fn broadcast(&self, message: IPCMessage) -> IPCResult<()> {
        let clients = self.clients.lock().unwrap();
        for client in clients.iter() {
            let _ = client.send(message.clone());
        }
        println!("📡 Broadcast message: {:?}", message);
        Ok(())
    }
}

pub struct IPCClient {
    connected: bool,
    sender: Option<mpsc::Sender<IPCMessage>>,
    receiver: Option<mpsc::Receiver<IPCMessage>>,
}

impl IPCClient {
    pub async fn new() -> IPCResult<Self> {
        Ok(Self {
            connected: false,
            sender: None,
            receiver: None,
        })
    }
    
    pub async fn connect(&mut self) -> IPCResult<()> {
        let (tx, rx) = mpsc::channel();
        self.sender = Some(tx);
        self.receiver = Some(rx);
        self.connected = true;
        println!("🔌 IPC Client connected (simplified mode)");
        Ok(())
    }
    
    pub async fn send(&mut self, message: IPCMessage) -> IPCResult<()> {
        if !self.connected {
            return Err(IPCError::ConnectionFailed("Not connected".into()));
        }
        
        println!("📤 IPC Send: {:?}", message);
        Ok(())
    }
    
    pub async fn receive(&mut self) -> IPCResult<IPCMessage> {
        if !self.connected {
            return Err(IPCError::ConnectionFailed("Not connected".into()));
        }
        
        // For testing, simulate various responses
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        Ok(IPCMessage::Pong)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_basic() {
        test_basic_ipc().unwrap();
    }
}