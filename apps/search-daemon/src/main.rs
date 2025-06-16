// R5 Flowlight Search Daemon - Headless Background Process
// Responsibilities:
// 1. Listen for global shortcuts (Cmd+Space / Ctrl+Space)
// 2. Manage IPC server for communication with main-app and overlay
// 3. Handle search requests and route to appropriate modules
// 4. Maintain application state and configuration

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use log::{info, error, warn, debug};
use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::{AppHandle, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
// use uuid::Uuid;

// Internal modules
mod daemon;
mod handlers;
mod state;

// use daemon::SearchDaemon;
use state::DaemonState;
use ipc_communication::{IPCServer, IPCMessage};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in daemon mode (detach from terminal)
    #[arg(short, long)]
    daemon: bool,
    
    /// Log level (error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    log_level: String,
    
    /// PID file location
    #[arg(short, long, default_value = "/tmp/r5-flowlight-daemon.pid")]
    pid_file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Initialize logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(&args.log_level)
    ).init();
    
    info!("R5 Flowlight Search Daemon starting...");
    
    // Daemonize if requested
    if args.daemon {
        daemonize_process(&args.pid_file)?;
    }
    
    // Setup signal handlers for graceful shutdown
    setup_signal_handlers().await?;
    
    // Initialize daemon state
    let daemon_state = Arc::new(RwLock::new(DaemonState::new()));
    
    // Start IPC server
    let mut ipc_server = IPCServer::new().await?;
    setup_ipc_handlers(&mut ipc_server, daemon_state.clone()).await?;
    ipc_server.start().await?;
    
    // Start Tauri headless app for global shortcuts
    let daemon_state_clone = daemon_state.clone();
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(move |app| {
            setup_global_shortcuts(app.handle(), daemon_state_clone)?;
            info!("Search daemon initialized successfully");
            Ok(())
        })
        .build(tauri::generate_context!())?
        .run(|_app, _event| {});
    
    Ok(())
}

fn daemonize_process(pid_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    use daemonize::Daemonize;
    
    let daemonize = Daemonize::new()
        .pid_file(pid_file)
        .chown_pid_file(true)
        .working_directory("/tmp")
        .user("nobody")
        .group("daemon")
        .umask(0o027)
        .privileged_action(|| {
            info!("Daemon privileges dropped");
        });
    
    match daemonize.start() {
        Ok(_) => {
            info!("Daemon started successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to daemonize: {}", e);
            Err(Box::new(e))
        }
    }
}

async fn setup_signal_handlers() -> Result<(), Box<dyn std::error::Error>> {
    use signal_hook::consts::signal::*;
    use signal_hook_tokio::Signals;
    use tokio_stream::StreamExt;
    
    let mut signals = Signals::new(&[SIGTERM, SIGINT, SIGQUIT])?;
    
    tokio::spawn(async move {
        while let Some(signal) = signals.next().await {
            match signal {
                SIGTERM | SIGINT | SIGQUIT => {
                    info!("Received shutdown signal: {}", signal);
                    graceful_shutdown().await;
                    std::process::exit(0);
                }
                _ => unreachable!(),
            }
        }
    });
    
    Ok(())
}

async fn graceful_shutdown() {
    info!("Performing graceful shutdown...");
    
    // Cleanup IPC connections
    // Cleanup temporary files
    // Save state if needed
    
    info!("Graceful shutdown completed");
}

fn setup_global_shortcuts<R: Runtime>(
    app_handle: &AppHandle<R>,
    daemon_state: Arc<RwLock<DaemonState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let global_shortcut = app_handle.global_shortcut();
    
    // Clear any existing shortcuts
    let _ = global_shortcut.unregister("CmdOrCtrl+Space");
    
    // Register the main shortcut
    let daemon_state_clone = daemon_state.clone();
    global_shortcut.on_shortcut("CmdOrCtrl+Space", move |_app, _shortcut, event| {
        if event.state == ShortcutState::Pressed {
            let state = daemon_state_clone.clone();
            tokio::spawn(async move {
                handle_global_shortcut(state).await;
            });
        }
    })?;
    
    info!("Global shortcut Cmd+Space registered successfully");
    Ok(())
}

async fn handle_global_shortcut(daemon_state: Arc<RwLock<DaemonState>>) {
    info!("Global shortcut triggered - toggling search overlay");
    
    let state = daemon_state.read().await;
    
    // Check if module is selected
    if state.current_module.is_none() {
        warn!("No module selected - showing module selection notification");
        // TODO: Show notification to select module
        return;
    }
    
    // Check for debouncing
    if state.is_processing_shortcut() {
        debug!("Shortcut already processing, ignoring");
        return;
    }
    
    drop(state); // Release read lock
    
    // Set processing flag
    {
        let mut state = daemon_state.write().await;
        state.set_processing_shortcut(true);
    }
    
    // Send toggle command via IPC (this will communicate with overlay)
    if let Err(e) = broadcast_ipc_message(IPCMessage::ToggleOverlay).await {
        error!("Failed to send toggle overlay message: {}", e);
    }
    
    // Reset processing flag after delay
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    {
        let mut state = daemon_state.write().await;
        state.set_processing_shortcut(false);
    }
}

async fn broadcast_ipc_message(message: IPCMessage) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: This will be implemented when IPC server is fully set up
    // For now, just log the message
    info!("Broadcasting IPC message: {:?}", message);
    Ok(())
}

async fn setup_ipc_handlers(
    ipc_server: &mut IPCServer,
    daemon_state: Arc<RwLock<DaemonState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Register ping handler
    ipc_server.register_handler("ping", handlers::PingHandler::new());
    
    // Register overlay control handlers
    ipc_server.register_handler("toggle_overlay", handlers::OverlayHandler::new());
    ipc_server.register_handler("show_overlay", handlers::OverlayHandler::new());
    ipc_server.register_handler("hide_overlay", handlers::OverlayHandler::new());
    
    // Register search handlers
    ipc_server.register_handler("search_query", handlers::SearchHandler::new(daemon_state.clone()));
    
    // Register module handlers
    ipc_server.register_handler("update_module", handlers::ModuleHandler::new(daemon_state.clone()));
    ipc_server.register_handler("get_current_module", handlers::ModuleHandler::new(daemon_state));
    
    info!("IPC handlers registered successfully");
    Ok(())
}