// R5 Flowlight REAL Search Overlay
// Production-ready visual interface with Tauri

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{info, error, warn, debug};
use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::{AppHandle, Manager, WebviewWindow, Emitter};
use serde::{Deserialize, Serialize};

#[cfg(target_os = "macos")]
use core_graphics::display::{CGDisplay, CGPoint};

use ipc_communication::{IPCClient, IPCMessage, TcpIPCClient};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealOverlayState {
    pub visible: bool,
    pub current_query: String,
    pub current_module: Option<String>,
    pub search_results: Vec<RealSearchResult>,
    pub selected_index: usize,
    pub loading: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealSearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: Option<String>,
    pub action_type: String,
    pub score: f32,
    pub module: String,
}

impl Default for RealOverlayState {
    fn default() -> Self {
        Self {
            visible: false,
            current_query: String::new(),
            current_module: None,
            search_results: Vec::new(),
            selected_index: 0,
            loading: false,
        }
    }
}

#[tauri::command]
async fn toggle_overlay(
    window: WebviewWindow,
    state: tauri::State<'_, Arc<RwLock<RealOverlayState>>>,
) -> Result<bool, String> {
    let mut overlay_state = state.write().await;
    
    if overlay_state.visible {
        hide_overlay_real(&window).await?;
        overlay_state.visible = false;
        overlay_state.current_query.clear();
        overlay_state.search_results.clear();
        overlay_state.selected_index = 0;
        info!("🪟 REAL Overlay hidden");
    } else {
        show_overlay_real(&window).await?;
        overlay_state.visible = true;
        info!("🪟 REAL Overlay shown - ready for input");
    }
    
    Ok(overlay_state.visible)
}

#[tauri::command]
async fn search_real_query(
    query: String,
    state: tauri::State<'_, Arc<RwLock<RealOverlayState>>>,
    ipc_client: tauri::State<'_, Arc<RwLock<TcpIPCClient>>>,
) -> Result<Vec<RealSearchResult>, String> {
    info!("🔍 REAL search for: '{}'", query);
    
    {
        let mut overlay_state = state.write().await;
        overlay_state.loading = true;
        overlay_state.current_query = query.clone();
    }
    
    let session_id = uuid::Uuid::new_v4().to_string();
    
    let mut client = ipc_client.write().await;
    let message = IPCMessage::SearchQuery {
        query: query.clone(),
        session_id: session_id.clone(),
    };
    
    if let Err(e) = client.send(message).await {
        error!("❌ Failed to send search query: {}", e);
        let mut overlay_state = state.write().await;
        overlay_state.loading = false;
        return Err("Failed to send search query".to_string());
    }

    info!("📤 Search request sent to daemon");
    let start_time = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(5);

    loop {
        if start_time.elapsed() > timeout {
            error!("⏰ Search timeout");
            break;
        }

        match client.receive().await {
            Ok(IPCMessage::SearchResults { results, session_id: resp_session }) => {
                if resp_session == session_id {
                    let real_results: Vec<RealSearchResult> = results.into_iter()
                        .enumerate()
                        .map(|(i, r)| RealSearchResult {
                            id: r.id,
                            title: r.title,
                            description: r.description,
                            icon: r.icon,
                            action_type: r.action_type,
                            score: 1.0 - (i as f32 * 0.1),
                            module: "daily".to_string(),
                        })
                        .collect();
                    
                    {
                        let mut overlay_state = state.write().await;
                        overlay_state.search_results = real_results.clone();
                        overlay_state.loading = false;
                        overlay_state.selected_index = 0;
                    }
                    
                    info!("✅ Received {} search results", real_results.len());
                    return Ok(real_results);
                }
            }
            Ok(other_msg) => {
                debug!("📨 Ignoring non-search message: {:?}", other_msg);
            }
            Err(e) => {
                error!("❌ Error receiving search results: {}", e);
                break;
            }
        }
    }

    {
        let mut overlay_state = state.write().await;
        overlay_state.loading = false;
    }
    
    Err("Search failed or timed out".to_string())
}

#[tauri::command]
async fn execute_action(
    result_id: String,
    state: tauri::State<'_, Arc<RwLock<RealOverlayState>>>,
) -> Result<(), String> {
    let overlay_state = state.read().await;
    
    if let Some(result) = overlay_state.search_results.iter().find(|r| r.id == result_id) {
        info!("⚡ Executing action for: {} ({})", result.title, result.action_type);
        // TODO: Implement actual file/app opening, launching, copying
        match result.action_type.as_str() {
            "open" => { info!("📂 Opening: {}", result.title); }
            "launch" => { info!("🚀 Launching: {}", result.title); }
            "copy" => { info!("📋 Copying: {}", result.title); }
            _ => { warn!("❓ Unknown action type: {}", result.action_type); }
        }
        Ok(())
    } else {
        Err(format!("Result with id '{}' not found", result_id))
    }
}

#[tauri::command]
async fn navigate_results(
    direction: String,
    state: tauri::State<'_, Arc<RwLock<RealOverlayState>>>,
) -> Result<usize, String> {
    let mut overlay_state = state.write().await;
    
    if overlay_state.search_results.is_empty() {
        return Ok(0);
    }
    
    match direction.as_str() {
        "up" => {
            if overlay_state.selected_index > 0 {
                overlay_state.selected_index -= 1;
            }
        }
        "down" => {
            if overlay_state.selected_index < overlay_state.search_results.len() - 1 {
                overlay_state.selected_index += 1;
            }
        }
        _ => {}
    }
    
    debug!("🧭 Navigation: {} -> index {}", direction, overlay_state.selected_index);
    Ok(overlay_state.selected_index)
}

#[tauri::command]
async fn get_overlay_state(
    state: tauri::State<'_, Arc<RwLock<RealOverlayState>>>,
) -> Result<RealOverlayState, String> {
    let overlay_state = state.read().await;
    Ok(overlay_state.clone())
}

#[tauri::command]
async fn get_selected_module() -> Result<String, String> {
    Ok("daily".to_string())
}

#[tauri::command] 
async fn search_files(_query: String) -> Result<Vec<RealSearchResult>, String> {
    let results = vec![
        RealSearchResult {
            id: "file-1".to_string(),
            title: format!("Result for '{}'", _query),
            description: "Mock file result".to_string(),
            icon: Some("📄".to_string()),
            action_type: "open".to_string(),
            score: 0.9,
            module: "daily".to_string(),
        }
    ];
    Ok(results)
}

#[tauri::command]
async fn search_apps(_query: String) -> Result<Vec<AppResult>, String> {
    let results = vec![
        AppResult {
            name: format!("App for '{}'", _query),
            path: "/Applications/MockApp.app".to_string(),
            icon: "🔧".to_string(),
        }
    ];
    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppResult {
    pub name: String,
    pub path: String, 
    pub icon: String,
}

#[tauri::command]
async fn ai_query(prompt: String, _context: String) -> Result<String, String> {
    Ok(format!("AI Response for '{}' in context '{}':\n\nThis is a mock response from the overlay AI system.", prompt, _context))
}

#[tauri::command]
async fn execute_module_function(module_id: String, function_id: String) -> Result<String, String> {
    Ok(format!("Executed function '{}' in module '{}'", function_id, module_id))
}

async fn show_overlay_real(window: &WebviewWindow) -> Result<(), String> {
    info!("🪟 Starting show_overlay_real function");
    // Simplificado: Apenas mostra, foca e define always_on_top. 
    // O posicionamento complexo será reintroduzido se necessário.
    let _ = window.set_always_on_top(true);
    let _ = window.show();
    let _ = window.set_focus();
    Ok(())
}

async fn hide_overlay_real(window: &WebviewWindow) -> Result<(), String> {
    let _ = window.hide();
    Ok(())
}

#[cfg(target_os = "macos")]
fn get_cursor_position() -> Option<(f64, f64)> {
    // Placeholder para macOS. A implementação real foi removida para simplificar.
    None
}

#[cfg(not(target_os = "macos"))]
fn get_cursor_position() -> Option<(f64, f64)> {
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    info!("🚀 R5 Flowlight REAL Search Overlay starting...");
    
    let overlay_state = Arc::new(RwLock::new(RealOverlayState::default()));
    
    // --- IPC Connection with Retry Logic ---
    let mut tcp_ipc_client = None;
    for i in 0..10 {
        info!("🔌 Attempting to connect to daemon... (Attempt {})", i + 1);
        match TcpIPCClient::new().await {
            Ok(client) => {
                info!("✅ TCP IPC Client created successfully");
                tcp_ipc_client = Some(Arc::new(RwLock::new(client)));
                break;
            }
            Err(e) => {
                 error!("❌ Instantiation failed: {}. Retrying in 1 second...", e);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    let tcp_ipc_client = match tcp_ipc_client {
        Some(client) => client,
        None => {
            let err_msg = "❌ Could not connect to the daemon after multiple attempts. Exiting.";
            error!("{}", err_msg);
            return Err(err_msg.into());
        }
    };
    
    tauri::Builder::default()
        .manage(overlay_state.clone())
        .manage(tcp_ipc_client.clone())
        .invoke_handler(tauri::generate_handler![
            toggle_overlay,
            search_real_query,
            execute_action,
            navigate_results,
            get_overlay_state,
            get_selected_module,
            search_files,
            search_apps,
            ai_query,
            execute_module_function
        ])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let overlay_state_clone = overlay_state.clone();
            let tcp_ipc_client_clone = tcp_ipc_client.clone();
            
            // Get the overlay window instance
            let overlay_window = app.get_webview_window("overlay").expect("Overlay window not found");

            tokio::spawn(async move {
                listen_for_real_daemon_messages(app_handle, overlay_state_clone, tcp_ipc_client_clone, overlay_window).await;
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    Ok(())
}

async fn listen_for_real_daemon_messages(
    app_handle: AppHandle,
    overlay_state: Arc<RwLock<RealOverlayState>>,
    tcp_ipc_client: Arc<RwLock<TcpIPCClient>>,
    overlay_window: WebviewWindow,
) {
    info!("👂 Starting REAL IPC message listener");
    
    loop {
        let message = {
            let mut client = tcp_ipc_client.write().await;
            match client.receive().await {
                Ok(msg) => Some(msg),
                Err(_) => None,
            }
        };
        
        if let Some(message) = message {
            info!("🔥 MESSAGE RECEIVED: {:?}", message);
            match message {
                IPCMessage::ToggleOverlay => {
                    info!("📨 Processing REAL ToggleOverlay from daemon");
                    // Use the passed overlay_window directly
                    let is_visible = overlay_window.is_visible().unwrap_or(false);
                    info!("🔍 Current window visibility: {}", is_visible);
                    
                    let mut state = overlay_state.write().await;
                    info!("🔍 Current state.visible: {}", state.visible);
                    
                    if state.visible || is_visible {
                        info!("🪟 Hiding overlay via IPC");
                        let _ = hide_overlay_real(&overlay_window).await;
                        state.visible = false;
                        state.current_query.clear();
                        state.search_results.clear();
                        state.selected_index = 0;
                        info!("✅ Overlay hidden and state updated");
                    } else {
                        info!("🪟 Showing overlay via IPC");
                        match show_overlay_real(&overlay_window).await {
                            Ok(_) => {
                                state.visible = true;
                                info!("✅ Overlay shown and state updated to visible");
                            }
                            Err(e) => {
                                error!("❌ Failed to show overlay: {}", e);
                            }
                        }
                    }
                    
                    // Emit event to frontend for state sync
                    let _ = overlay_window.emit("overlay-state-changed", &*state);
                    info!("📡 State change event emitted to frontend");
                }
                IPCMessage::ShowOverlay { query } => {
                    info!("📨 Processing REAL ShowOverlay: {:?}", query);
                    let _ = show_overlay_real(&overlay_window).await;
                    let mut state = overlay_state.write().await;
                    state.visible = true;
                    if let Some(q) = query {
                        state.current_query = q;
                    }
                    let _ = overlay_window.emit("overlay-state-changed", &*state);
                }
                IPCMessage::HideOverlay => {
                    info!("📨 Processing REAL HideOverlay from daemon");
                    let _ = hide_overlay_real(&overlay_window).await;
                    let mut state = overlay_state.write().await;
                    state.visible = false;
                    state.current_query.clear();
                    state.search_results.clear();
                    let _ = overlay_window.emit("overlay-state-changed", &*state);
                }
                IPCMessage::Ping => {
                    debug!("📨 Received ping from daemon, sending pong");
                    let mut client = tcp_ipc_client.write().await;
                    let _ = client.send(IPCMessage::Pong).await;
                }
                _ => {
                    debug!("📨 Received other message: {:?}", message);
                }
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}