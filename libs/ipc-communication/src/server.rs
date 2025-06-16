// IPC Server - Runs in search-daemon

use crate::{IPCMessage, IPCResult, IPCError, IPCSender, IPCReceiver};
use tokio::sync::mpsc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct IPCServer {
    sender: IPCSender,
    receiver: Option<IPCReceiver>,
    clients: Arc<RwLock<HashMap<String, IPCSender>>>,
    message_handlers: HashMap<String, Box<dyn MessageHandler + Send + Sync>>,
}

#[async_trait::async_trait]
pub trait MessageHandler {
    async fn handle(&self, message: IPCMessage, client_id: &str) -> IPCResult<Option<IPCMessage>>;
}

impl IPCServer {
    pub async fn new() -> IPCResult<Self> {
        let (sender, receiver) = crate::create_ipc_server().await?;
        
        Ok(IPCServer {
            sender,
            receiver: Some(receiver),
            clients: Arc::new(RwLock::new(HashMap::new())),
            message_handlers: HashMap::new(),
        })
    }
    
    pub fn register_handler<H>(&mut self, message_type: &str, handler: H) 
    where
        H: MessageHandler + Send + Sync + 'static
    {
        self.message_handlers.insert(message_type.to_string(), Box::new(handler));
    }
    
    pub async fn start(&mut self) -> IPCResult<()> {
        let mut receiver = self.receiver.take()
            .ok_or_else(|| IPCError::ConnectionFailed("No receiver available".to_string()))?;
        
        let clients = self.clients.clone();
        let handlers = Arc::new(self.message_handlers.drain().collect::<HashMap<_, _>>());
        
        tokio::spawn(async move {
            log::info!("IPC Server started");
            
            while let Some(message) = receiver.recv().await {
                Self::handle_message(message, &clients, &handlers).await;
            }
        });
        
        Ok(())
    }
    
    async fn handle_message(
        message: IPCMessage,
        clients: &Arc<RwLock<HashMap<String, IPCSender>>>,
        handlers: &Arc<HashMap<String, Box<dyn MessageHandler + Send + Sync>>>,
    ) {
        let message_type = Self::get_message_type(&message);
        let client_id = "default"; // TODO: Extract from actual client connection
        
        if let Some(handler) = handlers.get(&message_type) {
            match handler.handle(message, client_id).await {
                Ok(Some(response)) => {
                    Self::send_to_client(response, client_id, clients).await;
                }
                Ok(None) => {
                    // No response needed
                }
                Err(e) => {
                    log::error!("Handler error for {}: {}", message_type, e);
                }
            }
        } else {
            log::warn!("No handler registered for message type: {}", message_type);
        }
    }
    
    async fn send_to_client(
        message: IPCMessage,
        client_id: &str,
        clients: &Arc<RwLock<HashMap<String, IPCSender>>>,
    ) {
        let clients_guard = clients.read().await;
        if let Some(client_sender) = clients_guard.get(client_id) {
            if let Err(e) = client_sender.send(message) {
                log::error!("Failed to send message to client {}: {}", client_id, e);
            }
        }
    }
    
    pub async fn broadcast(&self, message: IPCMessage) -> IPCResult<()> {
        let clients = self.clients.read().await;
        for (client_id, sender) in clients.iter() {
            if let Err(e) = sender.send(message.clone()) {
                log::error!("Failed to broadcast to client {}: {}", client_id, e);
            }
        }
        Ok(())
    }
    
    fn get_message_type(message: &IPCMessage) -> String {
        match message {
            IPCMessage::ToggleOverlay => "toggle_overlay".to_string(),
            IPCMessage::ShowOverlay { .. } => "show_overlay".to_string(),
            IPCMessage::HideOverlay => "hide_overlay".to_string(),
            IPCMessage::SearchQuery { .. } => "search_query".to_string(),
            IPCMessage::SearchResults { .. } => "search_results".to_string(),
            IPCMessage::UpdateModule { .. } => "update_module".to_string(),
            IPCMessage::GetCurrentModule => "get_current_module".to_string(),
            IPCMessage::ModuleChanged { .. } => "module_changed".to_string(),
            IPCMessage::StartDaemon => "start_daemon".to_string(),
            IPCMessage::StopDaemon => "stop_daemon".to_string(),
            IPCMessage::DaemonStatus { .. } => "daemon_status".to_string(),
            IPCMessage::Ping => "ping".to_string(),
            IPCMessage::Pong => "pong".to_string(),
            IPCMessage::ClearResults => "clear_results".to_string(),
        }
    }
}

// Default handlers
pub struct PingHandler;

#[async_trait::async_trait]
impl MessageHandler for PingHandler {
    async fn handle(&self, message: IPCMessage, _client_id: &str) -> IPCResult<Option<IPCMessage>> {
        match message {
            IPCMessage::Ping => Ok(Some(IPCMessage::Pong)),
            _ => Ok(None),
        }
    }
}

pub struct OverlayControlHandler;

#[async_trait::async_trait]
impl MessageHandler for OverlayControlHandler {
    async fn handle(&self, message: IPCMessage, _client_id: &str) -> IPCResult<Option<IPCMessage>> {
        match message {
            IPCMessage::ToggleOverlay => {
                log::info!("Toggling overlay");
                // Implementation will be added when overlay is ready
                Ok(None)
            }
            IPCMessage::ShowOverlay { query } => {
                log::info!("Showing overlay with query: {:?}", query);
                // Implementation will be added when overlay is ready
                Ok(None)
            }
            IPCMessage::HideOverlay => {
                log::info!("Hiding overlay");
                // Implementation will be added when overlay is ready
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}