// R5 Flowlight Search Overlay - Global Search Interface
// Displays search UI when triggered by daemon
// Communicates with daemon via IPC

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{info, error, warn, debug};
use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::{AppHandle, Manager, Window, WindowBuilder, WindowUrl};
use serde::{Deserialize, Serialize};

use ipc_communication::{IPCClient, IPCMessage, IPCResult};
use shared_core::{AppConfig, Module};

// Platform-specific imports removed for now

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayState {
    pub visible: bool,
    pub current_query: String,
    pub current_module: Option<String>,
    pub search_results: Vec<SearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: Option<String>,
    pub action_type: String,
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            visible: false,
            current_query: String::new(),
            current_module: None,
            search_results: Vec::new(),
        }
    }
}

#[tauri::command]
async fn toggle_overlay(
    window: Window,
    state: tauri::State<'_, Arc<RwLock<OverlayState>>>,
) -> Result<bool, String> {
    let mut overlay_state = state.write().await;
    
    if overlay_state.visible {
        hide_overlay_window(&window).await?;
        overlay_state.visible = false;
        info!("Overlay hidden");
    } else {
        show_overlay_window(&window).await?;
        overlay_state.visible = true;
        info!("Overlay shown");
    }
    
    Ok(overlay_state.visible)
}

#[tauri::command]
async fn search_query(
    query: String,
    state: tauri::State<'_, Arc<RwLock<OverlayState>>>,
    ipc_client: tauri::State<'_, Arc<RwLock<IPCClient>>>,
) -> Result<Vec<SearchResult>, String> {
    info!("Searching for: {}", query);
    
    let session_id = uuid::Uuid::new_v4().to_string();
    
    // Send search request to daemon
    let mut client = ipc_client.write().await;
    let message = IPCMessage::SearchQuery {
        query: query.clone(),
        session_id: session_id.clone(),
    };
    
    match client.send(message).await {
        Ok(_) => {
            // Wait for response
            match client.receive().await {
                Ok(IPCMessage::SearchResults { results, session_id: resp_session }) => {
                    if resp_session == session_id {
                        let search_results: Vec<SearchResult> = results.into_iter().map(|r| {
                            SearchResult {
                                id: r.id,
                                title: r.title,
                                description: r.description,
                                icon: r.icon,
                                action_type: r.action_type,
                            }
                        }).collect();
                        
                        // Update state
                        let mut overlay_state = state.write().await;
                        overlay_state.current_query = query;
                        overlay_state.search_results = search_results.clone();
                        
                        Ok(search_results)
                    } else {
                        Err("Session ID mismatch".to_string())
                    }
                }
                Ok(_) => Err("Unexpected response from daemon".to_string()),
                Err(e) => Err(format!("Failed to receive search results: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to send search query: {}", e)),
    }
}

#[tauri::command]
async fn hide_overlay(
    window: Window,
    state: tauri::State<'_, Arc<RwLock<OverlayState>>>,
) -> Result<(), String> {
    hide_overlay_window(&window).await?;
    
    let mut overlay_state = state.write().await;
    overlay_state.visible = false;
    overlay_state.current_query.clear();
    overlay_state.search_results.clear();
    
    info!("Overlay hidden and cleared");
    Ok(())
}

async fn show_overlay_window(window: &Window) -> Result<(), String> {
    // Position overlay at center-top of screen
    let _ = window.set_always_on_top(true);
    let _ = window.set_skip_taskbar(true);
    let _ = window.set_focus();
    let _ = window.show();
    
    Ok(())
}

async fn hide_overlay_window(window: &Window) -> Result<(), String> {
    let _ = window.hide();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    info!("R5 Flowlight Search Overlay starting...");
    
    // Initialize overlay state
    let overlay_state = Arc::new(RwLock::new(OverlayState::default()));
    
    // Initialize IPC client
    let ipc_client = Arc::new(RwLock::new(
        IPCClient::new().await
            .map_err(|e| format!("Failed to create IPC client: {}", e))?
    ));
    
    // Connect to daemon
    {
        let mut client = ipc_client.write().await;
        client.connect().await
            .map_err(|e| format!("Failed to connect to daemon: {}", e))?;
        info!("Connected to search daemon via IPC");
    }
    
    tauri::Builder::default()
        .manage(overlay_state)
        .manage(ipc_client)
        .invoke_handler(tauri::generate_handler![
            toggle_overlay,
            search_query,
            hide_overlay
        ])
        .setup(|app| {
            let main_window = WindowBuilder::new(
                app,
                "main",
                WindowUrl::default()
            )
            .title("R5 Flowlight Search")
            .inner_size(400.0, 60.0)
            .min_inner_size(400.0, 60.0)
            .resizable(false)
            .decorations(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .visible(false)
            .center()
            .build()?;
            
            info!("Search overlay window created");
            
            // Setup IPC listener for daemon messages
            let app_handle = app.handle();
            let overlay_state = app_handle.state::<Arc<RwLock<OverlayState>>>();
            let ipc_client = app_handle.state::<Arc<RwLock<IPCClient>>>();
            
            tokio::spawn(async move {
                listen_for_daemon_messages(app_handle, overlay_state, ipc_client).await;
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    Ok(())
}

async fn listen_for_daemon_messages(
    app_handle: AppHandle,
    overlay_state: tauri::State<'_, Arc<RwLock<OverlayState>>>,
    ipc_client: tauri::State<'_, Arc<RwLock<IPCClient>>>,
) {
    info!("Starting IPC message listener");
    
    loop {
        let mut client = match ipc_client.try_write() {
            Ok(client) => client,
            Err(_) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }
        };
        
        match client.receive().await {
            Ok(message) => {
                match message {
                    IPCMessage::ToggleOverlay => {
                        info!("Received toggle overlay command from daemon");
                        if let Some(window) = app_handle.get_window("main") {
                            let _ = toggle_overlay(window, overlay_state.clone()).await;
                        }
                    }
                    IPCMessage::ShowOverlay { query } => {
                        info!("Received show overlay command: {:?}", query);
                        if let Some(window) = app_handle.get_window("main") {
                            let _ = show_overlay_window(&window).await;
                            let mut state = overlay_state.write().await;
                            state.visible = true;
                            if let Some(q) = query {
                                state.current_query = q;
                            }
                        }
                    }
                    IPCMessage::HideOverlay => {
                        info!("Received hide overlay command from daemon");
                        if let Some(window) = app_handle.get_window("main") {
                            let _ = hide_overlay(window, overlay_state.clone()).await;
                        }
                    }
                    IPCMessage::Ping => {
                        debug!("Received ping from daemon");
                        if let Err(e) = client.send(IPCMessage::Pong).await {
                            error!("Failed to send pong: {}", e);
                        }
                    }
                    _ => {
                        debug!("Received other message: {:?}", message);
                    }
                }
            }
            Err(e) => {
                warn!("IPC receive error: {} - retrying in 1s", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}