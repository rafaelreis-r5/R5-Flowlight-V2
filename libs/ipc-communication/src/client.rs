// IPC Client - Used by main-app and search-overlay

use crate::{IPCMessage, IPCResult, IPCError, IPCSender, IPCReceiver, IPC_TIMEOUT_MS};
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct IPCClient {
    sender: IPCSender,
    receiver: Option<IPCReceiver>,
    response_handlers: Arc<RwLock<Vec<Box<dyn ResponseHandler + Send + Sync>>>>,
}

#[async_trait::async_trait]
pub trait ResponseHandler {
    async fn handle_response(&self, message: IPCMessage) -> bool; // returns true if handled
}

impl IPCClient {
    pub async fn connect() -> IPCResult<Self> {
        let (sender, receiver) = crate::create_ipc_client().await?;
        
        Ok(IPCClient {
            sender,
            receiver: Some(receiver),
            response_handlers: Arc::new(RwLock::new(Vec::new())),
        })
    }
    
    pub async fn start_listening(&mut self) -> IPCResult<()> {
        let mut receiver = self.receiver.take()
            .ok_or_else(|| IPCError::ConnectionFailed("No receiver available".to_string()))?;
        
        let handlers = self.response_handlers.clone();
        
        tokio::spawn(async move {
            log::info!("IPC Client listening for responses");
            
            while let Some(message) = receiver.recv().await {
                let handlers_guard = handlers.read().await;
                
                for handler in handlers_guard.iter() {
                    if handler.handle_response(message.clone()).await {
                        break; // Message was handled
                    }
                }
            }
        });
        
        Ok(())
    }
    
    pub fn add_response_handler<H>(&self, handler: H) 
    where
        H: ResponseHandler + Send + Sync + 'static
    {
        let handlers = self.response_handlers.clone();
        tokio::spawn(async move {
            let mut handlers_guard = handlers.write().await;
            handlers_guard.push(Box::new(handler));
        });
    }
    
    pub async fn send(&self, message: IPCMessage) -> IPCResult<()> {
        self.sender.send(message)
            .map_err(|e| IPCError::SendFailed(e.to_string()))
    }
    
    pub async fn send_with_timeout(&self, message: IPCMessage) -> IPCResult<()> {
        timeout(Duration::from_millis(IPC_TIMEOUT_MS), self.send(message)).await
            .map_err(|_| IPCError::SendFailed("Timeout".to_string()))?
    }
    
    // Convenience methods for common operations
    pub async fn toggle_overlay(&self) -> IPCResult<()> {
        self.send(IPCMessage::ToggleOverlay).await
    }
    
    pub async fn show_overlay(&self, query: Option<String>) -> IPCResult<()> {
        self.send(IPCMessage::ShowOverlay { query }).await
    }
    
    pub async fn hide_overlay(&self) -> IPCResult<()> {
        self.send(IPCMessage::HideOverlay).await
    }
    
    pub async fn search(&self, query: String, session_id: String) -> IPCResult<()> {
        self.send(IPCMessage::SearchQuery { query, session_id }).await
    }
    
    pub async fn update_module(&self, module_id: String) -> IPCResult<()> {
        self.send(IPCMessage::UpdateModule { module_id }).await
    }
    
    pub async fn get_current_module(&self) -> IPCResult<()> {
        self.send(IPCMessage::GetCurrentModule).await
    }
    
    pub async fn ping(&self) -> IPCResult<()> {
        self.send(IPCMessage::Ping).await
    }
    
    pub async fn start_daemon(&self) -> IPCResult<()> {
        self.send(IPCMessage::StartDaemon).await
    }
    
    pub async fn stop_daemon(&self) -> IPCResult<()> {
        self.send(IPCMessage::StopDaemon).await
    }
    
    pub async fn check_daemon_status(&self) -> IPCResult<()> {
        self.send(IPCMessage::DaemonStatus { running: false, pid: None }).await
    }
}

// Default response handlers
pub struct SearchResultsHandler<F> 
where
    F: Fn(Vec<crate::SearchResult>) + Send + Sync + 'static,
{
    callback: F,
}

impl<F> SearchResultsHandler<F>
where
    F: Fn(Vec<crate::SearchResult>) + Send + Sync + 'static,
{
    pub fn new(callback: F) -> Self {
        Self { callback }
    }
}

#[async_trait::async_trait]
impl<F> ResponseHandler for SearchResultsHandler<F>
where
    F: Fn(Vec<crate::SearchResult>) + Send + Sync + 'static,
{
    async fn handle_response(&self, message: IPCMessage) -> bool {
        match message {
            IPCMessage::SearchResults { results, .. } => {
                (self.callback)(results);
                true
            }
            _ => false,
        }
    }
}

pub struct ModuleChangeHandler<F>
where
    F: Fn(String) + Send + Sync + 'static,
{
    callback: F,
}

impl<F> ModuleChangeHandler<F>
where
    F: Fn(String) + Send + Sync + 'static,
{
    pub fn new(callback: F) -> Self {
        Self { callback }
    }
}

#[async_trait::async_trait]
impl<F> ResponseHandler for ModuleChangeHandler<F>
where
    F: Fn(String) + Send + Sync + 'static,
{
    async fn handle_response(&self, message: IPCMessage) -> bool {
        match message {
            IPCMessage::ModuleChanged { module_id } => {
                (self.callback)(module_id);
                true
            }
            _ => false,
        }
    }
}

pub struct DaemonStatusHandler<F>
where
    F: Fn(bool, Option<u32>) + Send + Sync + 'static,
{
    callback: F,
}

impl<F> DaemonStatusHandler<F>
where
    F: Fn(bool, Option<u32>) + Send + Sync + 'static,
{
    pub fn new(callback: F) -> Self {
        Self { callback }
    }
}

#[async_trait::async_trait]
impl<F> ResponseHandler for DaemonStatusHandler<F>
where
    F: Fn(bool, Option<u32>) + Send + Sync + 'static,
{
    async fn handle_response(&self, message: IPCMessage) -> bool {
        match message {
            IPCMessage::DaemonStatus { running, pid } => {
                (self.callback)(running, pid);
                true
            }
            _ => false,
        }
    }
}