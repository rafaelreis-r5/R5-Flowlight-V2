// Search Daemon Core Implementation

use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error, warn};

use crate::state::DaemonState;
use ipc_communication::{IPCServer, IPCMessage};

pub struct SearchDaemon {
    state: Arc<RwLock<DaemonState>>,
    ipc_server: Option<IPCServer>,
}

impl SearchDaemon {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(DaemonState::new())),
            ipc_server: None,
        }
    }
    
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing search daemon...");
        
        // Initialize IPC server
        let mut ipc_server = IPCServer::new().await?;
        self.setup_ipc_handlers(&mut ipc_server).await?;
        ipc_server.start().await?;
        
        self.ipc_server = Some(ipc_server);
        
        info!("Search daemon initialized successfully");
        Ok(())
    }
    
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting search daemon services...");
        
        // Start background tasks
        self.start_background_tasks().await?;
        
        info!("Search daemon started successfully");
        Ok(())
    }
    
    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Shutting down search daemon...");
        
        // Cleanup operations
        self.cleanup().await?;
        
        info!("Search daemon shutdown completed");
        Ok(())
    }
    
    pub fn get_state(&self) -> Arc<RwLock<DaemonState>> {
        self.state.clone()
    }
    
    async fn setup_ipc_handlers(&self, ipc_server: &mut IPCServer) -> Result<(), Box<dyn std::error::Error>> {
        use crate::handlers::*;
        
        // Register all handlers
        ipc_server.register_handler("ping", PingHandler::new());
        ipc_server.register_handler("toggle_overlay", OverlayHandler::new());
        ipc_server.register_handler("show_overlay", OverlayHandler::new());
        ipc_server.register_handler("hide_overlay", OverlayHandler::new());
        ipc_server.register_handler("search_query", SearchHandler::new(self.state.clone()));
        ipc_server.register_handler("clear_results", SearchHandler::new(self.state.clone()));
        ipc_server.register_handler("update_module", ModuleHandler::new(self.state.clone()));
        ipc_server.register_handler("get_current_module", ModuleHandler::new(self.state.clone()));
        ipc_server.register_handler("daemon_status", DaemonControlHandler::new(self.state.clone()));
        ipc_server.register_handler("stop_daemon", DaemonControlHandler::new(self.state.clone()));
        
        info!("IPC handlers registered successfully");
        Ok(())
    }
    
    async fn start_background_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Start stats monitoring task
        let state_clone = self.state.clone();
        tokio::spawn(async move {
            Self::stats_monitoring_task(state_clone).await;
        });
        
        // Start cleanup task
        let state_clone = self.state.clone();
        tokio::spawn(async move {
            Self::cleanup_task(state_clone).await;
        });
        
        info!("Background tasks started");
        Ok(())
    }
    
    async fn stats_monitoring_task(state: Arc<RwLock<DaemonState>>) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
        
        loop {
            interval.tick().await;
            
            // Update memory usage stats
            if let Ok(memory_usage) = Self::get_memory_usage() {
                let mut state_guard = state.write().await;
                state_guard.update_stats(memory_usage);
            }
        }
    }
    
    async fn cleanup_task(state: Arc<RwLock<DaemonState>>) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
        
        loop {
            interval.tick().await;
            
            // Cleanup expired search sessions
            {
                let mut state_guard = state.write().await;
                if let Some(session_id) = &state_guard.search_session_id {
                    // Check if session is too old
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64;
                    
                    let elapsed = now.saturating_sub(state_guard.stats.last_activity);
                    if elapsed > 300000 { // 5 minutes
                        warn!("Cleaning up expired search session: {}", session_id);
                        state_guard.end_search_session();
                    }
                }
            }
        }
    }
    
    fn get_memory_usage() -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        // Get current process memory usage
        #[cfg(unix)]
        {
            use std::fs;
            let status = fs::read_to_string("/proc/self/status")?;
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return Ok(parts[1].parse::<u64>().unwrap_or(0));
                    }
                }
            }
        }
        
        #[cfg(windows)]
        {
            // TODO: Implement Windows memory usage detection
            return Ok(0);
        }
        
        Ok(0)
    }
    
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Save current state if needed
        let state = self.state.read().await;
        info!("Final daemon stats:");
        info!("  Shortcuts triggered: {}", state.stats.shortcuts_triggered);
        info!("  Searches performed: {}", state.stats.searches_performed);
        info!("  Memory usage: {} KB", state.stats.memory_usage_kb);
        
        // Additional cleanup operations can be added here
        
        Ok(())
    }
}

impl Drop for SearchDaemon {
    fn drop(&mut self) {
        info!("Search daemon instance dropped");
    }
}