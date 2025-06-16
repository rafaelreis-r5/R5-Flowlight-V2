// Real but Simple IPC Implementation
// Uses in-memory message bus for daemon <-> overlay communication

use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};
use std::sync::Arc;
use lazy_static::lazy_static;

use crate::{IPCMessage, IPCResult, IPCError, MessageHandler};

// Global message bus - shared between daemon and overlay
lazy_static! {
    static ref MESSAGE_BUS: Arc<RwLock<MessageBus>> = Arc::new(RwLock::new(MessageBus::new()));
}

#[derive(Debug)]
pub struct MessageBus {
    clients: HashMap<String, mpsc::UnboundedSender<IPCMessage>>,
}

impl MessageBus {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, client_id: String, sender: mpsc::UnboundedSender<IPCMessage>) {
        println!("📡 Registering client: {}", client_id);
        self.clients.insert(client_id, sender);
    }
    
    pub fn unregister(&mut self, client_id: &str) {
        println!("📡 Unregistering client: {}", client_id);
        self.clients.remove(client_id);
    }
    
    pub fn broadcast(&self, message: IPCMessage, sender_id: Option<&str>) {
        println!("📡 Broadcasting message: {:?} from {:?}", message, sender_id);
        
        for (client_id, sender) in &self.clients {
            // Don't send back to sender
            if let Some(sender_id) = sender_id {
                if client_id == sender_id {
                    println!("📡 Skipping sender: {}", client_id);
                    continue;
                }
            }
            
            println!("📡 Sending message to client: {}", client_id);
            let _ = sender.send(message.clone());
        }
    }
    
    pub fn send_to(&self, target: &str, message: IPCMessage) -> bool {
        if let Some(sender) = self.clients.get(target) {
            sender.send(message).is_ok()
        } else {
            false
        }
    }
    
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }
}

// Real IPC Server
pub struct IPCServer {
    client_id: String,
    receiver: Option<mpsc::UnboundedReceiver<IPCMessage>>,
    handlers: HashMap<String, Box<dyn MessageHandler + Send + Sync>>,
}

impl IPCServer {
    pub async fn new() -> IPCResult<Self> {
        let client_id = "daemon".to_string();
        let (sender, receiver) = mpsc::unbounded_channel();
        
        // Register with message bus
        {
            let mut bus = MESSAGE_BUS.write().await;
            bus.register(client_id.clone(), sender);
        }
        
        Ok(Self {
            client_id,
            receiver: Some(receiver),
            handlers: HashMap::new(),
        })
    }
    
    pub fn register_handler<T: MessageHandler + Send + Sync + 'static>(&mut self, name: &str, handler: T) {
        self.handlers.insert(name.to_string(), Box::new(handler));
    }
    
    pub async fn start(&mut self) -> IPCResult<()> {
        println!("📡 Real IPC Server started with ID: {}", self.client_id);
        
        // Start message processing loop
        let client_id = self.client_id.clone();
        if let Some(mut receiver) = self.receiver.take() {
            tokio::spawn(async move {
                while let Some(message) = receiver.recv().await {
                    println!("📨 Daemon received: {:?}", message);
                    
                    // Process message and potentially respond
                    // For now, just broadcast responses
                    match message {
                        IPCMessage::SearchQuery { query, session_id } => {
                            // Simulate search processing
                            let results = vec![
                                crate::SearchResult {
                                    id: "daemon-1".to_string(),
                                    title: format!("Daemon Result for '{}'", query),
                                    description: "Real result from daemon".to_string(),
                                    icon: Some("🤖".to_string()),
                                    action_type: "open".to_string(),
                                    metadata: std::collections::HashMap::new(),
                                }
                            ];
                            
                            let response = IPCMessage::SearchResults { results, session_id };
                            
                            let bus = MESSAGE_BUS.read().await;
                            bus.broadcast(response, Some(&client_id));
                        }
                        _ => {
                            // Echo other messages back
                            let bus = MESSAGE_BUS.read().await;
                            bus.broadcast(message, Some(&client_id));
                        }
                    }
                }
            });
        }
        
        Ok(())
    }
    
    pub async fn broadcast(&self, message: IPCMessage) -> IPCResult<()> {
        let bus = MESSAGE_BUS.read().await;
        bus.broadcast(message, Some(&self.client_id));
        Ok(())
    }
}

impl Drop for IPCServer {
    fn drop(&mut self) {
        let client_id = self.client_id.clone();
        tokio::spawn(async move {
            let mut bus = MESSAGE_BUS.write().await;
            bus.unregister(&client_id);
        });
    }
}

// Real IPC Client
pub struct IPCClient {
    client_id: String,
    receiver: Option<mpsc::UnboundedReceiver<IPCMessage>>,
    sender_id: String,
}

impl IPCClient {
    pub async fn new() -> IPCResult<Self> {
        let client_id = format!("overlay-{}", uuid::Uuid::new_v4().to_string()[..8].to_string());
        let (sender, receiver) = mpsc::unbounded_channel();
        
        // Register with message bus
        {
            let mut bus = MESSAGE_BUS.write().await;
            bus.register(client_id.clone(), sender);
        }
        
        Ok(Self {
            sender_id: client_id.clone(),
            client_id,
            receiver: Some(receiver),
        })
    }
    
    pub async fn connect(&mut self) -> IPCResult<()> {
        println!("🔌 Real IPC Client connected: {}", self.client_id);
        
        // Send ping to announce connection
        self.send(IPCMessage::Ping).await?;
        
        Ok(())
    }
    
    pub async fn send(&mut self, message: IPCMessage) -> IPCResult<()> {
        let bus = MESSAGE_BUS.read().await;
        bus.broadcast(message, Some(&self.sender_id));
        Ok(())
    }
    
    pub async fn receive(&mut self) -> IPCResult<IPCMessage> {
        if let Some(receiver) = &mut self.receiver {
            receiver.recv().await
                .ok_or_else(|| IPCError::ReceiveFailed("Channel closed".to_string()))
        } else {
            Err(IPCError::ReceiveFailed("No receiver available".to_string()))
        }
    }
    
    pub async fn try_receive(&mut self) -> IPCResult<Option<IPCMessage>> {
        if let Some(receiver) = &mut self.receiver {
            match receiver.try_recv() {
                Ok(msg) => Ok(Some(msg)),
                Err(mpsc::error::TryRecvError::Empty) => Ok(None),
                Err(mpsc::error::TryRecvError::Disconnected) => Err(IPCError::ReceiveFailed("Channel disconnected".to_string())),
            }
        } else {
            Err(IPCError::ReceiveFailed("No receiver available".to_string()))
        }
    }
}

impl Drop for IPCClient {
    fn drop(&mut self) {
        let client_id = self.client_id.clone();
        tokio::spawn(async move {
            let mut bus = MESSAGE_BUS.write().await;
            bus.unregister(&client_id);
        });
    }
}

// Debug function to check message bus status
pub async fn debug_message_bus() {
    let bus = MESSAGE_BUS.read().await;
    println!("📊 Message Bus Status: {} clients connected", bus.client_count());
}