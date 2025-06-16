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
// AppConfig import removed as it's not used

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
    ipc_client: tauri::State<'_, Arc<RwLock<IPCClient>>>,
) -> Result<Vec<RealSearchResult>, String> {
    info!("🔍 REAL search for: '{}'", query);
    
    // Update state to show loading
    {
        let mut overlay_state = state.write().await;
        overlay_state.loading = true;
        overlay_state.current_query = query.clone();
    }
    
    let session_id = uuid::Uuid::new_v4().to_string();
    
    // Send REAL search request to daemon
    let mut client = ipc_client.write().await;
    let message = IPCMessage::SearchQuery {
        query: query.clone(),
        session_id: session_id.clone(),
    };
    
    match client.send(message).await {
        Ok(_) => {
            info!("📤 Search request sent to daemon");
            
            // Wait for response with timeout
            let start_time = std::time::Instant::now();
            let timeout = std::time::Duration::from_secs(5);
            
            loop {
                if start_time.elapsed() > timeout {
                    error!("⏰ Search timeout");
                    break;
                }
                
                match client.try_receive().await {
                    Ok(Some(IPCMessage::SearchResults { results, session_id: resp_session })) => {
                        if resp_session == session_id {
                            let real_results: Vec<RealSearchResult> = results.into_iter()
                                .enumerate()
                                .map(|(i, r)| RealSearchResult {
                                    id: r.id,
                                    title: r.title,
                                    description: r.description,
                                    icon: r.icon,
                                    action_type: r.action_type,
                                    score: 1.0 - (i as f32 * 0.1), // Simple scoring
                                    module: "daily".to_string(), // TODO: Get from daemon
                                })
                                .collect();
                            
                            // Update state with real results
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
                    Ok(Some(_)) => {
                        // Ignore other messages
                        debug!("📨 Ignoring non-search message");
                    }
                    Ok(None) => {
                        // No message available, wait a bit
                        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    }
                    Err(e) => {
                        error!("❌ Error receiving search results: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            error!("❌ Failed to send search query: {}", e);
        }
    }
    
    // Clear loading state on error
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
        
        match result.action_type.as_str() {
            "open" => {
                info!("📂 Opening: {}", result.title);
                // TODO: Implement actual file/app opening
            }
            "launch" => {
                info!("🚀 Launching: {}", result.title);
                // TODO: Implement app launching
            }
            "copy" => {
                info!("📋 Copying: {}", result.title);
                // TODO: Implement clipboard operations
            }
            _ => {
                warn!("❓ Unknown action type: {}", result.action_type);
            }
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
    // Return default module for overlay
    Ok("daily".to_string())
}

#[tauri::command] 
async fn search_files(query: String) -> Result<Vec<RealSearchResult>, String> {
    // Mock file search for now - later integrate with real search
    let results = vec![
        RealSearchResult {
            id: "file-1".to_string(),
            title: format!("Result for '{}'", query),
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
async fn search_apps(query: String) -> Result<Vec<AppResult>, String> {
    // Mock app search for now
    let results = vec![
        AppResult {
            name: format!("App for '{}'", query),
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
async fn ai_query(prompt: String, context: String) -> Result<String, String> {
    // Mock AI response for now
    Ok(format!("AI Response for '{}' in context '{}':\n\nThis is a mock response from the overlay AI system.", prompt, context))
}

#[tauri::command]
async fn execute_module_function(module_id: String, function_id: String) -> Result<String, String> {
    // Mock module function execution
    Ok(format!("Executed function '{}' in module '{}'", function_id, module_id))
}

async fn show_overlay_real(window: &WebviewWindow) -> Result<(), String> {
    info!("🪟 Starting show_overlay_real function");
    
    // 🎯 Get cursor position to determine target monitor
    let cursor_pos = get_cursor_position();
    info!("🖱️  Cursor position: {:?}", cursor_pos);
    
    // Get all available monitors to find the one containing cursor
    if let Ok(monitors) = window.available_monitors() {
        let mut target_monitor = None;
        
        if let Some((cursor_x, cursor_y)) = cursor_pos {
            // Find which monitor contains the cursor
            for monitor in &monitors {
                let monitor_pos = monitor.position();
                let monitor_size = monitor.size();
                let scale = monitor.scale_factor();
                
                // Calculate monitor bounds
                let monitor_left = monitor_pos.x as f64;
                let monitor_top = monitor_pos.y as f64;
                let monitor_right = monitor_left + (monitor_size.width as f64 / scale);
                let monitor_bottom = monitor_top + (monitor_size.height as f64 / scale);
                
                info!("🖥️  Monitor {:?}: bounds ({}, {}) to ({}, {})", 
                      monitor.name(), monitor_left, monitor_top, monitor_right, monitor_bottom);
                
                // Check if cursor is within this monitor's bounds
                if cursor_x >= monitor_left && cursor_x <= monitor_right &&
                   cursor_y >= monitor_top && cursor_y <= monitor_bottom {
                    target_monitor = Some(monitor);
                    info!("🎯 Found cursor on monitor: {:?}", monitor.name());
                    break;
                }
            }
        }
        
        // Fallback to first monitor if cursor detection failed
        if target_monitor.is_none() {
            target_monitor = monitors.get(0);
            info!("🔄 Using fallback monitor: {:?}", target_monitor.map(|m| m.name()));
        }
        
        if let Some(monitor) = target_monitor {
            let size = monitor.size();
            let position = monitor.position();
            let scale = monitor.scale_factor();
            
            // Full screen overlay window (invisible background)
            let window_width = 1400.0;
            let window_height = 900.0;
            
            // Center the large invisible window on the target monitor
            let x = position.x as f64 + (size.width as f64 / scale - window_width) / 2.0;
            let y = position.y as f64 + (size.height as f64 / scale - window_height) / 2.0;
            
            info!("🖥️  Target monitor: {:?}", monitor.name());
            info!("📏 Monitor size: {}x{}, scale: {}, position: {:?}", size.width, size.height, scale, position);
            info!("📍 Positioning overlay window at: x={}, y={}", x, y);
            
            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { 
                x: x as i32, 
                y: y as i32 
            }));
            
            // Set the large window size for full overlay coverage
            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: window_width as u32,
                height: window_height as u32,
            }));
        }
    } else {
        // Ultimate fallback to current monitor
        if let Ok(monitor) = window.current_monitor() {
            if let Some(monitor) = monitor {
                let size = monitor.size();
                let scale = monitor.scale_factor();
                
                let window_width = 1400.0;
                let window_height = 900.0;
                
                let x = (size.width as f64 / scale - window_width) / 2.0;
                let y = (size.height as f64 / scale - window_height) / 2.0;
                
                info!("🖥️  Ultimate fallback monitor size: {}x{}, scale: {}", size.width, size.height, scale);
                info!("📍 Ultimate fallback positioning window at: x={}, y={}", x, y);
                
                let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { 
                    x: x as i32, 
                    y: y as i32 
                }));
                
                let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                    width: window_width as u32,
                    height: window_height as u32,
                }));
            }
        }
    }
    
    // Set window properties for true overlay behavior (like Spotlight)
    let _ = window.set_always_on_top(true);
    let _ = window.set_skip_taskbar(true);
    let _ = window.set_decorations(false);
    
    // Use Tauri's safe APIs instead of raw objc calls
    info!("🎯 Setting window properties with Tauri APIs");
    
    // Show the window first
    match window.show() {
        Ok(_) => info!("✅ Window show() called successfully"),
        Err(e) => error!("❌ Failed to call window.show(): {}", e),
    }
    
    // Force to front and focus - crucial for appearing over all apps
    match window.set_focus() {
        Ok(_) => info!("✅ Window focus set successfully"), 
        Err(e) => error!("❌ Failed to set window focus: {}", e),
    }
    
    // Double-ensure always on top (sometimes needed on macOS)
    let _ = window.set_always_on_top(true);
    
    // Additional focus attempts for stubborn cases
    let _ = window.set_focus();
    
    // Small delay and try again to ensure it appears over everything
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let _ = window.set_always_on_top(true);
    let _ = window.set_focus();
    
    info!("🪟 show_overlay_real completed");
    Ok(())
}

async fn hide_overlay_real(window: &WebviewWindow) -> Result<(), String> {
    let _ = window.hide();
    Ok(())
}

#[cfg(target_os = "macos")]
fn get_cursor_position() -> Option<(f64, f64)> {
    use objc::{msg_send, sel, sel_impl, class};
    use objc::runtime::{Object};
    use std::ptr;
    
    unsafe {
        // Get NSEvent class
        let nsevent_class = class!(NSEvent);
        
        // Call [NSEvent mouseLocation] to get cursor position
        let mouse_location: cocoa::foundation::NSPoint = msg_send![nsevent_class, mouseLocation];
        
        info!("🖱️  Raw mouse location: x={}, y={}", mouse_location.x, mouse_location.y);
        
        // NSEvent mouseLocation returns coordinates in screen coordinates
        // where (0,0) is bottom-left, but we need top-left coordinates
        
        // Get main screen to convert coordinates
        let nsscreen_class = class!(NSScreen);
        let main_screen: *mut Object = msg_send![nsscreen_class, mainScreen];
        
        if !main_screen.is_null() {
            let screen_frame: cocoa::foundation::NSRect = msg_send![main_screen, frame];
            
            // Convert from bottom-left to top-left coordinate system
            let converted_y = screen_frame.size.height - mouse_location.y;
            
            info!("🖱️  Converted cursor position: x={}, y={}", mouse_location.x, converted_y);
            
            Some((mouse_location.x, converted_y))
        } else {
            info!("⚠️  Could not get main screen");
            Some((mouse_location.x, mouse_location.y))
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn get_cursor_position() -> Option<(f64, f64)> {
    None // Fallback for non-macOS platforms
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    info!("🚀 R5 Flowlight REAL Search Overlay starting...");
    
    // Initialize REAL overlay state
    let overlay_state = Arc::new(RwLock::new(RealOverlayState::default()));
    
    // Initialize TCP IPC client for cross-process communication
    let tcp_ipc_client = Arc::new(RwLock::new(
        TcpIPCClient::new().await
            .map_err(|e| format!("Failed to create TCP IPC client: {}", e))?
    ));
    
    // TCP connection is established automatically, no manual connect needed
    info!("✅ TCP IPC Client connected to daemon");
    
    // TCP connection already established
    
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
            // Remove menus from overlay app (safe approach)
            #[cfg(target_os = "macos")]
            {
                let _ = app.set_menu(tauri::menu::Menu::new(app.handle()).unwrap());
            }
            
            info!("🪟 REAL search overlay window created");
            
            // Verify the window exists
            if let Some(window) = app.get_webview_window("overlay") {
                info!("✅ Main window found: {}", window.label());
                
                // Use safe Tauri APIs only
                let _ = window.set_always_on_top(true);
                let _ = window.set_skip_taskbar(true);
                let _ = window.set_decorations(false);
                
                info!("✅ Window properties set with safe APIs");
            } else {
                error!("❌ Main window not found!");
            }
            
            // Setup REAL IPC listener for daemon messages
            let app_handle = app.handle().clone();
            let overlay_state = overlay_state.clone();
            let tcp_ipc_client = tcp_ipc_client.clone();
            
            info!("🚀 Starting IPC message listener task...");
            tokio::spawn(async move {
                listen_for_real_daemon_messages(app_handle, overlay_state, tcp_ipc_client).await;
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
) {
    info!("👂 Starting REAL IPC message listener");
    
    loop {
        // Use try_receive with timeout instead of blocking receive
        let message = {
            let mut client = match tcp_ipc_client.try_write() {
                Ok(client) => client,
                Err(_) => {
                    debug!("🔒 Could not acquire write lock on TCP IPC client");
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    continue;
                }
            };
            
            // Use timeout-based receive for TCP IPC
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(100), 
                client.receive()
            ).await {
                Ok(Ok(msg)) => {
                    info!("📨 Received TCP IPC message: {:?}", msg);
                    Some(msg)
                },
                Ok(Err(e)) => {
                    error!("📨 TCP IPC receive error: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                    None
                }
                Err(_) => {
                    // Timeout - no message available
                    debug!("📭 No TCP messages available (timeout)");
                    None
                }
            }
        };
        
        if let Some(message) = message {
            info!("🔥 MESSAGE RECEIVED: {:?}", message);
            match message {
                IPCMessage::ToggleOverlay => {
                    info!("📨 Processing REAL ToggleOverlay from daemon");
                    if let Some(window) = app_handle.get_webview_window("overlay") {
                        info!("✅ Found main window: {}", window.label());
                        
                        // Check if window is currently visible
                        let is_visible = window.is_visible().unwrap_or(false);
                        info!("🔍 Current window visibility: {}", is_visible);
                        
                        let mut state = overlay_state.write().await;
                        info!("🔍 Current state.visible: {}", state.visible);
                        
                        if state.visible || is_visible {
                            info!("🪟 Hiding overlay via IPC");
                            let _ = hide_overlay_real(&window).await;
                            state.visible = false;
                            state.current_query.clear();
                            state.search_results.clear();
                            state.selected_index = 0;
                            info!("✅ Overlay hidden and state updated");
                        } else {
                            info!("🪟 Showing overlay via IPC");
                            match show_overlay_real(&window).await {
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
                        let _ = window.emit("overlay-state-changed", &*state);
                        info!("📡 State change event emitted to frontend");
                    } else {
                        error!("❌ Could not find main window!");
                    }
                }
                IPCMessage::ShowOverlay { query } => {
                    info!("📨 Processing REAL ShowOverlay: {:?}", query);
                    if let Some(window) = app_handle.get_webview_window("overlay") {
                        let _ = show_overlay_real(&window).await;
                        let mut state = overlay_state.write().await;
                        state.visible = true;
                        if let Some(q) = query {
                            state.current_query = q;
                        }
                        let _ = window.emit("overlay-state-changed", &*state);
                    }
                }
                IPCMessage::HideOverlay => {
                    info!("📨 Processing REAL HideOverlay from daemon");
                    if let Some(window) = app_handle.get_webview_window("overlay") {
                        let _ = hide_overlay_real(&window).await;
                        let mut state = overlay_state.write().await;
                        state.visible = false;
                        state.current_query.clear();
                        state.search_results.clear();
                        let _ = window.emit("overlay-state-changed", &*state);
                    }
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
        
        // Small delay to prevent excessive CPU usage
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}