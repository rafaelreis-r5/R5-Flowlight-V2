// Simple Search Overlay - Testing without Tauri complexity
// Connects to daemon via IPC

use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error, warn, debug};
use clap::Parser;

use ipc_communication::{IPCClient, IPCMessage};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Log level (error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[derive(Debug, Clone)]
pub struct OverlayState {
    pub visible: bool,
    pub current_query: String,
    pub search_results: Vec<SearchResult>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: Option<String>,
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            visible: false,
            current_query: String::new(),
            search_results: Vec::new(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Initialize logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(&args.log_level)
    ).init();
    
    info!("🪟 R5 Flowlight Simple Search Overlay starting...");
    
    // Initialize overlay state
    let overlay_state = Arc::new(RwLock::new(OverlayState::default()));
    
    // Connect to daemon
    let mut ipc_client = IPCClient::new().await?;
    ipc_client.connect().await?;
    info!("📡 Connected to search daemon via IPC");
    
    // Start listening for daemon messages
    let overlay_state_clone = overlay_state.clone();
    tokio::spawn(async move {
        listen_for_daemon_messages(overlay_state_clone).await;
    });
    
    // Simulate user interactions
    simulate_user_interactions(overlay_state.clone()).await?;
    
    Ok(())
}

async fn listen_for_daemon_messages(overlay_state: Arc<RwLock<OverlayState>>) {
    info!("👂 Starting IPC message listener for daemon commands");
    
    // Simulate receiving messages from daemon
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(11));
    
    loop {
        interval.tick().await;
        
        // Simulate receiving ToggleOverlay from daemon
        info!("📨 Received ToggleOverlay message from daemon");
        
        let mut state = overlay_state.write().await;
        state.visible = !state.visible;
        
        if state.visible {
            info!("🪟 ✅ Overlay now VISIBLE - ready for search");
            info!("💡 User can now type search queries...");
        } else {
            info!("🪟 ❌ Overlay now HIDDEN");
            state.current_query.clear();
            state.search_results.clear();
        }
    }
}

async fn simulate_user_interactions(overlay_state: Arc<RwLock<OverlayState>>) -> Result<(), Box<dyn std::error::Error>> {
    info!("⌨️  Simulating user interactions...");
    
    // Wait for overlay to be visible
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        let state = overlay_state.read().await;
        if state.visible {
            drop(state);
            break;
        }
    }
    
    // Simulate user typing
    let search_queries = vec!["test", "hello", "search", "app"];
    
    for query in search_queries {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        let state = overlay_state.read().await;
        if !state.visible {
            drop(state);
            info!("🚫 Overlay not visible, skipping search");
            continue;
        }
        drop(state);
        
        info!("⌨️  User typing: '{}'", query);
        
        // Update overlay state
        {
            let mut state = overlay_state.write().await;
            state.current_query = query.to_string();
        }
        
        // Send search request to daemon (simulated)
        let search_message = IPCMessage::SearchQuery {
            query: query.to_string(),
            session_id: format!("overlay-{}", uuid::Uuid::new_v4()),
        };
        
        info!("📤 Sending search request to daemon: {:?}", search_message);
        
        // Simulate receiving search results from daemon
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        
        let mock_results = vec![
            SearchResult {
                id: "1".to_string(),
                title: format!("Result 1 for '{}'", query),
                description: "Mock result from overlay".to_string(),
                icon: Some("🔍".to_string()),
            },
            SearchResult {
                id: "2".to_string(),
                title: format!("Result 2 for '{}'", query),
                description: "Another mock result".to_string(),
                icon: Some("📄".to_string()),
            },
        ];
        
        {
            let mut state = overlay_state.write().await;
            state.search_results = mock_results.clone();
        }
        
        info!("📊 Displaying {} search results:", mock_results.len());
        for (i, result) in mock_results.iter().enumerate() {
            info!("   {}. {} {} - {}", i + 1, result.icon.as_ref().unwrap_or(&"•".to_string()), result.title, result.description);
        }
        
        info!("✨ Search overlay updated with results");
    }
    
    // Keep overlay running
    info!("🏃 Overlay running... Press Ctrl+C to stop");
    
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
    
    info!("✅ Overlay shutdown completed");
    Ok(())
}