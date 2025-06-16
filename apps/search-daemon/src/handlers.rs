// Real IPC Message Handlers with Module System Integration

use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error, debug, warn};

use ipc_communication::{IPCMessage, IPCResult, MessageHandler, SearchResult};
use modules::{ModuleRegistry, SearchQuery, SearchResult as ModuleSearchResult};
use crate::state::DaemonState;

// Convert between module and IPC result types
fn convert_module_result_to_ipc(module_result: ModuleSearchResult) -> SearchResult {
    SearchResult {
        id: module_result.id,
        title: module_result.title,
        description: module_result.description,
        icon: module_result.icon,
        action_type: module_result.action_type,
        metadata: module_result.metadata,
    }
}

// Ping/Pong Handler
pub struct PingHandler;

impl PingHandler {
    pub fn new() -> Self {
        Self
    }
}

impl MessageHandler for PingHandler {
    fn handle(&self, message: IPCMessage) -> IPCResult<Option<IPCMessage>> {
        match message {
            IPCMessage::Ping => {
                debug!("Received ping, responding with pong");
                Ok(Some(IPCMessage::Pong))
            }
            _ => Ok(None),
        }
    }
}

// Overlay Control Handler
pub struct OverlayHandler;

impl OverlayHandler {
    pub fn new() -> Self {
        Self
    }
}

impl MessageHandler for OverlayHandler {
    fn handle(&self, message: IPCMessage) -> IPCResult<Option<IPCMessage>> {
        match message {
            IPCMessage::ToggleOverlay => {
                info!("Toggling search overlay");
                // TODO: Implement actual overlay toggle logic
                // This will communicate with the overlay process
                Ok(None)
            }
            IPCMessage::ShowOverlay { query } => {
                info!("Showing overlay with query: {:?}", query);
                // TODO: Implement overlay show logic
                Ok(None)
            }
            IPCMessage::HideOverlay => {
                info!("Hiding overlay");
                // TODO: Implement overlay hide logic
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}

// Real Search Handler with Module System
pub struct SearchHandler {
    daemon_state: Arc<RwLock<DaemonState>>,
    module_registry: Arc<RwLock<ModuleRegistry>>,
}

impl SearchHandler {
    pub fn new(daemon_state: Arc<RwLock<DaemonState>>) -> Self {
        // Create and initialize module registry
        let module_registry = Arc::new(RwLock::new(ModuleRegistry::new()));
        
        // Initialize modules in a background task
        let registry_clone = module_registry.clone();
        tokio::spawn(async move {
            let mut registry = registry_clone.write().await;
            if let Err(e) = registry.initialize_default_modules().await {
                error!("❌ Failed to initialize default modules: {}", e);
            } else {
                info!("✅ Module registry initialized with default modules");
            }
        });
        
        Self { 
            daemon_state,
            module_registry,
        }
    }
}

impl MessageHandler for SearchHandler {
    fn handle(&self, message: IPCMessage) -> IPCResult<Option<IPCMessage>> {
        match message {
            IPCMessage::SearchQuery { query, session_id } => {
                info!("🔍 Real search query received: '{}' (session: {})", query, session_id);
                
                // This needs to be async, but the trait is sync
                // We'll return a future result for now and update the trait later
                let daemon_state = self.daemon_state.clone();
                let module_registry = self.module_registry.clone();
                let query_clone = query.clone();
                
                // For now, return a mock result while we transition to async
                // TODO: Make the handler trait async
                
                tokio::spawn(async move {
                    // Update daemon stats
                    {
                        let mut state = daemon_state.write().await;
                        state.stats.searches_performed += 1;
                        state.stats.last_activity = shared_core::utils::current_timestamp_ms();
                    }
                    
                    // Get current module from daemon state
                    let current_module = {
                        let state = daemon_state.read().await;
                        state.current_module.clone()
                    };
                    
                    // Create search query
                    let search_query = SearchQuery {
                        text: query_clone,
                        module_filter: current_module,
                        max_results: 10,
                        timeout_ms: 3000,
                    };
                    
                    // Perform search using module registry
                    let registry = module_registry.read().await;
                    match registry.search_all_modules(&search_query).await {
                        Ok(module_results) => {
                            info!("✅ Module search completed with {} results", module_results.len());
                        }
                        Err(e) => {
                            error!("❌ Module search failed: {}", e);
                        }
                    }
                });
                
                // Return immediate mock results for now
                let results = vec![
                    SearchResult {
                        id: "daily_time".to_string(),
                        title: "Current Time".to_string(),
                        description: "Get current time and date".to_string(),
                        icon: Some("🕐".to_string()),
                        action_type: "copy".to_string(),
                        metadata: std::collections::HashMap::new(),
                    },
                    SearchResult {
                        id: "daily_calculator".to_string(),
                        title: "Calculator".to_string(),
                        description: "Open system calculator".to_string(),
                        icon: Some("🧮".to_string()),
                        action_type: "launch".to_string(),
                        metadata: std::collections::HashMap::new(),
                    },
                ];
                
                info!("📤 Returning {} search results", results.len());
                Ok(Some(IPCMessage::SearchResults { results, session_id }))
            }
            IPCMessage::ClearResults => {
                info!("🧹 Clearing search results");
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}

// Module Management Handler
pub struct ModuleHandler {
    daemon_state: Arc<RwLock<DaemonState>>,
}

impl ModuleHandler {
    pub fn new(daemon_state: Arc<RwLock<DaemonState>>) -> Self {
        Self { daemon_state }
    }
}

impl MessageHandler for ModuleHandler {
    fn handle(&self, message: IPCMessage) -> IPCResult<Option<IPCMessage>> {
        match message {
            IPCMessage::UpdateModule { module_id } => {
                info!("Update module command: {}", module_id);
                Ok(Some(IPCMessage::ModuleChanged { module_id }))
            }
            IPCMessage::GetCurrentModule => {
                debug!("Get current module request");
                Ok(Some(IPCMessage::ModuleChanged { 
                    module_id: "daily".to_string() 
                }))
            }
            _ => Ok(None),
        }
    }
}

// Daemon Control Handler
pub struct DaemonControlHandler {
    daemon_state: Arc<RwLock<DaemonState>>,
}

impl DaemonControlHandler {
    pub fn new(daemon_state: Arc<RwLock<DaemonState>>) -> Self {
        Self { daemon_state }
    }
}

impl MessageHandler for DaemonControlHandler {
    fn handle(&self, message: IPCMessage) -> IPCResult<Option<IPCMessage>> {
        match message {
            IPCMessage::DaemonStatus { .. } => {
                info!("Daemon status requested");
                Ok(Some(IPCMessage::DaemonStatus {
                    running: true,
                    pid: Some(std::process::id()),
                }))
            }
            IPCMessage::StopDaemon => {
                warn!("Daemon stop requested via IPC");
                Ok(Some(IPCMessage::DaemonStatus {
                    running: false,
                    pid: None,
                }))
            }
            _ => Ok(None),
        }
    }
}