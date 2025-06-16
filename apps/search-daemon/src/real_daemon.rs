// R5 Flowlight REAL Search Daemon
// Production-ready implementation with real global shortcuts

use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error, debug, warn};
use clap::Parser;
use tauri::{AppHandle, Runtime, Manager, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

mod state;
mod handlers;

use state::DaemonState;
use ipc_communication::{IPCServer, IPCMessage, debug_message_bus, TcpIPCServer};
use shared_core::{ConfigManager};

// Global TCP IPC server instance for broadcasting
static mut GLOBAL_TCP_IPC_SERVER: Option<Arc<RwLock<TcpIPCServer>>> = None;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in daemon mode (detach from terminal)
    #[arg(short, long)]
    daemon: bool,
    
    /// Log level (error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    log_level: String,
    
    /// Custom shortcut (default: CmdOrCtrl+Space)
    #[arg(short, long, default_value = "CmdOrCtrl+Space")]
    shortcut: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Initialize logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(&args.log_level)
    ).init();
    
    info!("🚀 R5 Flowlight REAL Search Daemon starting...");
    
    // Initialize configuration manager
    let mut config_manager = ConfigManager::new()
        .map_err(|e| format!("Failed to initialize config manager: {}", e))?;
    
    // Use shortcut from config if not provided via CLI
    let shortcut = if args.shortcut == "CmdOrCtrl+Space" {
        config_manager.get_global_shortcut().to_string()
    } else {
        // Update config with CLI shortcut
        config_manager.set_global_shortcut(args.shortcut.clone())
            .map_err(|e| format!("Failed to save shortcut config: {}", e))?;
        args.shortcut.clone()
    };
    
    info!("⌨️  Global shortcut: {}", shortcut);
    info!("📁 Config path: {:?}", config_manager.get_config_file_path());
    
    // Initialize daemon state with real configuration
    let daemon_state = Arc::new(RwLock::new(DaemonState::new()));
    
    // Configure with available modules
    {
        let mut state = daemon_state.write().await;
        state.set_current_module("daily".to_string());
        info!("✅ Module 'daily' configured as default");
        debug_message_bus().await;
    }
    
    // Start TCP IPC server for cross-process communication
    let mut tcp_ipc_server = TcpIPCServer::new().await?;
    tcp_ipc_server.start().await?;
    
    // Store TCP server globally for broadcasting
    let tcp_server_arc = Arc::new(RwLock::new(tcp_ipc_server));
    unsafe {
        GLOBAL_TCP_IPC_SERVER = Some(tcp_server_arc.clone());
    }
    
    info!("📡 TCP IPC Server started and stored globally - ready for overlay connections");
    
    // Start background tasks
    start_background_services(daemon_state.clone()).await;
    
    // Start Tauri app with REAL global shortcuts
    let daemon_state_clone = daemon_state.clone();
    let shortcut_clone = shortcut.clone();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(move |app| {
            setup_real_global_shortcuts(app.handle(), daemon_state_clone, &shortcut_clone)?;
            info!("✅ REAL global shortcuts registered successfully");
            Ok(())
        })
        .build(tauri::generate_context!())?
        .run(|_app, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                info!("🛑 Daemon shutdown requested");
                api.prevent_exit();
            }
        });
    
    Ok(())
}

fn setup_real_global_shortcuts<R: Runtime>(
    app_handle: &AppHandle<R>,
    daemon_state: Arc<RwLock<DaemonState>>,
    shortcut: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let global_shortcut = app_handle.global_shortcut();
    
    // Clear any existing shortcuts
    let _ = global_shortcut.unregister_all();
    
    // Register the REAL shortcut
    let daemon_state_clone = daemon_state.clone();
    let app_handle_clone = app_handle.clone();
    
    global_shortcut.on_shortcut(shortcut, move |_app, _shortcut, event| {
        if event.state == ShortcutState::Pressed {
            let state = daemon_state_clone.clone();
            let app = app_handle_clone.clone();
            
            tokio::spawn(async move {
                handle_real_global_shortcut(state, app).await;
            });
        }
    })?;
    
    info!("🎯 REAL global shortcut '{}' registered and active", shortcut);
    Ok(())
}

async fn handle_real_global_shortcut<R: Runtime>(
    daemon_state: Arc<RwLock<DaemonState>>,
    app_handle: AppHandle<R>,
) {
    info!("🔥 REAL global shortcut triggered!");
    
    // Update daemon statistics
    {
        let mut state = daemon_state.write().await;
        state.stats.shortcuts_triggered += 1;
        state.stats.last_activity = shared_core::utils::current_timestamp_ms();
        debug!("📊 Shortcut count: {}", state.stats.shortcuts_triggered);
    }
    
    // Try TCP IPC first, if no overlay is connected, spawn one
    info!("🔄 Sending ToggleOverlay directly to overlay via IPC");
    
    match broadcast_real_ipc_message(IPCMessage::ToggleOverlay).await {
        Ok(_) => {
            info!("📡 Toggle overlay broadcasted to all connected clients");
        }
        Err(e) => {
            warn!("⚠️ No overlay connected, attempting to spawn overlay: {}", e);
            
            // Spawn overlay process
            match spawn_overlay_process().await {
                Ok(_) => {
                    info!("🚀 Overlay process spawned successfully");
                    
                    // Wait a bit for overlay to start, then try again
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    
                    if let Err(e) = broadcast_real_ipc_message(IPCMessage::ShowOverlay { query: None }).await {
                        error!("❌ Failed to communicate with spawned overlay: {}", e);
                    }
                }
                Err(e) => {
                    error!("❌ Failed to spawn overlay process: {}", e);
                }
            }
        }
    }
}

async fn broadcast_real_ipc_message(message: IPCMessage) -> Result<(), String> {
    info!("📤 Broadcasting REAL IPC message via TCP: {:?}", message);
    
    // Get the global TCP IPC server instance
    unsafe {
        if let Some(server_ref) = &GLOBAL_TCP_IPC_SERVER {
            let server = server_ref.read().await;
            if let Err(e) = server.broadcast(message).await {
                error!("❌ Failed to broadcast TCP message: {}", e);
                return Err(e.to_string());
            }
            info!("✅ TCP Message successfully broadcasted to all connected clients");
        } else {
            error!("❌ Global TCP IPC server not initialized");
            return Err("TCP IPC server not available".to_string());
        }
    }
    
    Ok(())
}

async fn setup_real_ipc_handlers(
    ipc_server: &mut IPCServer,
    daemon_state: Arc<RwLock<DaemonState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    use handlers::*;
    
    // Register REAL handlers with enhanced functionality
    ipc_server.register_handler("ping", PingHandler::new());
    ipc_server.register_handler("toggle_overlay", OverlayHandler::new());
    ipc_server.register_handler("show_overlay", OverlayHandler::new());
    ipc_server.register_handler("hide_overlay", OverlayHandler::new());
    
    // Enhanced search handler with real search capabilities
    ipc_server.register_handler("search_query", SearchHandler::new(daemon_state.clone()));
    ipc_server.register_handler("clear_results", SearchHandler::new(daemon_state.clone()));
    
    // Module management handlers
    ipc_server.register_handler("update_module", ModuleHandler::new(daemon_state.clone()));
    ipc_server.register_handler("get_current_module", ModuleHandler::new(daemon_state.clone()));
    ipc_server.register_handler("list_modules", ModuleHandler::new(daemon_state.clone()));
    
    // System control handlers
    ipc_server.register_handler("daemon_status", DaemonControlHandler::new(daemon_state.clone()));
    ipc_server.register_handler("daemon_stats", DaemonControlHandler::new(daemon_state));
    
    info!("🔧 REAL IPC handlers registered with enhanced functionality");
    Ok(())
}

async fn start_background_services(daemon_state: Arc<RwLock<DaemonState>>) {
    info!("🏗️  Starting background services...");
    
    // Stats monitoring service
    let state_clone = daemon_state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
        
        loop {
            interval.tick().await;
            
            let mut state = state_clone.write().await;
            state.stats.uptime_seconds += 60;
            
            // Log periodic stats
            if state.stats.uptime_seconds % 300 == 0 { // Every 5 minutes
                info!("📊 Daemon Stats - Uptime: {}s, Shortcuts: {}, Searches: {}", 
                      state.stats.uptime_seconds,
                      state.stats.shortcuts_triggered,
                      state.stats.searches_performed);
            }
        }
    });
    
    // Module health check service
    let state_clone = daemon_state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
        
        loop {
            interval.tick().await;
            
            let state = state_clone.read().await;
            if let Some(module) = &state.current_module {
                debug!("🔍 Health check for module: {}", module);
                // TODO: Implement module health checks
            }
        }
    });
    
    // IPC connection monitor
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
        
        loop {
            interval.tick().await;
            debug_message_bus().await;
        }
    });
    
    info!("✅ Background services started successfully");
}

async fn spawn_overlay_process() -> Result<(), String> {
    info!("🚀 Spawning overlay process...");
    
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let overlay_path = current_dir.join("target/debug/real-overlay");
    
    if !overlay_path.exists() {
        return Err(format!("Overlay binary not found at: {}", overlay_path.display()));
    }
    
    let child = std::process::Command::new(&overlay_path)
        .spawn()
        .map_err(|e| format!("Failed to spawn overlay process: {}", e))?;
    
    info!("🎯 Overlay process spawned with PID: {}", child.id());
    Ok(())
}