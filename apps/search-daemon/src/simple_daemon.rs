// Simple Search Daemon - Testing without Tauri complexity
// Just IPC communication and basic structure

use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error, warn, debug};
use clap::Parser;

mod state;
mod handlers;

use state::DaemonState;
use ipc_communication::{IPCServer, IPCMessage};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Log level (error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Initialize logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(&args.log_level)
    ).init();
    
    info!("🚀 R5 Flowlight Simple Search Daemon starting...");
    
    // Initialize daemon state with test module
    let daemon_state = Arc::new(RwLock::new(DaemonState::new()));
    
    // Configure test module
    {
        let mut state = daemon_state.write().await;
        state.set_current_module("daily".to_string());
        info!("✅ Test module 'daily' configured as default");
    }
    
    // Start IPC server
    let mut ipc_server = IPCServer::new().await?;
    setup_ipc_handlers(&mut ipc_server, daemon_state.clone()).await?;
    ipc_server.start().await?;
    
    info!("📡 IPC Server started successfully");
    
    // Simulate global shortcut functionality
    info!("⌨️  Global shortcut simulation: Press Ctrl+C to trigger");
    simulate_global_shortcuts(daemon_state.clone()).await?;
    
    Ok(())
}

async fn setup_ipc_handlers(
    ipc_server: &mut IPCServer,
    daemon_state: Arc<RwLock<DaemonState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    use handlers::*;
    
    // Register handlers
    ipc_server.register_handler("ping", PingHandler::new());
    ipc_server.register_handler("toggle_overlay", OverlayHandler::new());
    ipc_server.register_handler("search_query", SearchHandler::new(daemon_state.clone()));
    ipc_server.register_handler("update_module", ModuleHandler::new(daemon_state));
    
    info!("✅ IPC handlers registered successfully");
    Ok(())
}

async fn simulate_global_shortcuts(
    daemon_state: Arc<RwLock<DaemonState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    
    // Simulate periodic shortcut trigger
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
    
    tokio::spawn(async move {
        loop {
            interval.tick().await;
            
            info!("🔥 Simulating Cmd+Space global shortcut trigger");
            
            // Check daemon state
            let state = daemon_state.read().await;
            if state.current_module.is_none() {
                warn!("⚠️  No module selected - would show module selection");
            } else {
                info!("✨ Would toggle search overlay for module: {:?}", state.current_module);
                info!("🔍 Ready to handle search queries in '{}' module", state.current_module.as_ref().unwrap());
            }
            drop(state);
            
            // Simulate sending IPC message
            let message = IPCMessage::ToggleOverlay;
            info!("📡 Broadcasting IPC message: {:?}", message);
            
            // In real implementation, this would send to overlay
            info!("🪟 Overlay would appear/hide now");
            
            // Simulate search query after 2 seconds
            let daemon_state_search = daemon_state.clone();
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                simulate_search_query(daemon_state_search).await;
            });
        }
    });
    
    // Keep daemon running
    info!("🏃 Daemon running... Press Ctrl+C to stop");
    
    // Setup signal handlers for graceful shutdown
    use signal_hook::consts::signal::*;
    use signal_hook_tokio::Signals;
    use tokio_stream::StreamExt;
    
    let mut signals = Signals::new(&[SIGTERM, SIGINT, SIGQUIT])?;
    
    while let Some(signal) = signals.next().await {
        match signal {
            SIGTERM | SIGINT | SIGQUIT => {
                info!("🛑 Received shutdown signal: {}", signal);
                break;
            }
            _ => unreachable!(),
        }
    }
    
    info!("✅ Daemon shutdown completed");
    Ok(())
}

async fn simulate_search_query(daemon_state: Arc<RwLock<DaemonState>>) {
    info!("⌨️  Simulating user typing: 'test'");
    
    let module = {
        let state = daemon_state.read().await;
        state.current_module.as_ref().unwrap().clone()
    };
    
    // Simulate search request
    let search_message = IPCMessage::SearchQuery {
        query: "test".to_string(),
        session_id: "sim-123".to_string(),
    };
    
    info!("🔍 Simulating search in module '{}': {:?}", module, search_message);
    
    // Simulate search handler response (normally would come from handlers)
    let results = vec![
        ipc_communication::SearchResult {
            id: "1".to_string(),
            title: format!("Test Result 1 ({})", module),
            description: "Demo result from daemon".to_string(),
            icon: Some("🔍".to_string()),
            action_type: "open".to_string(),
            metadata: std::collections::HashMap::new(),
        },
        ipc_communication::SearchResult {
            id: "2".to_string(),
            title: format!("Test Result 2 ({})", module),
            description: "Another demo result".to_string(),
            icon: Some("📄".to_string()),
            action_type: "open".to_string(),
            metadata: std::collections::HashMap::new(),
        },
    ];
    
    let response = IPCMessage::SearchResults {
        results,
        session_id: "sim-123".to_string(),
    };
    
    info!("📊 Search completed, returning results: {:?}", response);
    info!("🎯 Would send results back to overlay for display");
}