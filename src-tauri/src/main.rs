// src-tauri/src/main.rs - VERSÃO FINALÍSSIMA CORRIGIDA PARA TAURI V2

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Módulos do projeto
mod core;
mod api;
mod modules;
mod ai;
mod utils;
pub mod commands;

// --- Imports Corrigidos para Tauri v2 ---
use tauri::{AppHandle, Manager, State, WebviewUrl, WebviewWindowBuilder, Runtime, Emitter};
use tauri::tray::TrayIconEvent;
// CORREÇÃO: Importa o `GlobalShortcutManagerExt` trait, que adiciona o método .global_shortcut_manager()
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Builder as GlobalShortcutBuilder, ShortcutState};
use log::{info, error};

use crate::core::search_engine::SearchEngine;
use crate::utils::logger;
// Settings commands imports removed as they're called directly in invoke_handler

#[derive(Default)]
pub struct AppState {
    pub selected_module: std::sync::Mutex<Option<String>>,
    pub search_window_visible: std::sync::Mutex<bool>,
    pub shortcut_processing: std::sync::Mutex<bool>,
}

fn load_dotenv() -> Result<(), String> {
    dotenvy::dotenv().map_err(|e| format!("Erro ao carregar .env: {}", e))?;
    let required_vars = ["SUPABASE_URL", "SUPABASE_SERVICE_KEY", "SUPABASE_ANON_KEY", "OPENAI_API_KEY"];
    for var in &required_vars {
        std::env::var(var).map_err(|_| format!("Variável de ambiente {} não definida", var))?;
    }
    Ok(())
}

// --- Seus Comandos Tauri ---
#[tauri::command]
async fn search_files(query: String) -> Result<Vec<api::search::SearchResult>, String> {
    api::search::search_files_handler(query).await
}

#[tauri::command]
async fn search_apps(query: String) -> Result<Vec<api::search::AppResult>, String> {
    api::search::search_apps_handler(query).await
}

#[tauri::command]
async fn get_module_data(module_id: String) -> Result<serde_json::Value, String> {
    api::modules::get_module_data_handler(module_id).await
}

#[tauri::command]
async fn ai_query(prompt: String, context: String) -> Result<String, String> {
    api::ai::ai_query_handler(prompt, context).await
}

#[tauri::command]
async fn set_selected_module(state: State<'_, AppState>, module_id: String) -> Result<(), String> {
    let mut selected_module = state.selected_module.lock().unwrap();
    *selected_module = Some(module_id.clone());
    info!("Selected module set to: {}", module_id);
    Ok(())
}

#[tauri::command]
async fn get_selected_module(state: State<'_, AppState>) -> Result<String, String> {
    let selected_module = state.selected_module.lock().unwrap();
    match selected_module.as_ref() {
        Some(module) => Ok(module.clone()),
        None => Ok("general".to_string()),
    }
}

#[tauri::command]
async fn toggle_search_launcher<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    info!("=== TOGGLE SEARCH LAUNCHER CALLED ===");
    let state: State<AppState> = app.state();
    
    // Improved debouncing to prevent double triggers
    {
        let mut processing = state.shortcut_processing.lock().unwrap();
        if *processing {
            info!("Shortcut already processing, skipping");
            return Ok(());
        }
        *processing = true;
        info!("Processing flag set to true");
    }
    
    // Check if module is selected
    let selected_module = state.selected_module.lock().unwrap().clone();
    if selected_module.is_none() {
        // Reset processing flag immediately
        *state.shortcut_processing.lock().unwrap() = false;
        
        // Show main window with warning
        if let Some(main_window) = app.get_webview_window("main") {
            main_window.show().map_err(|e| e.to_string())?;
            main_window.set_focus().map_err(|e| e.to_string())?;
            let _ = main_window.emit("show_module_warning", "Escolha um módulo/nicho para usar o Flowlight");
            info!("Main window shown with module selection warning");
        }
        
        return Ok(());
    }
    
    // Toggle search window (the floating bar)
    info!("Looking for search window...");
    if let Some(search_window) = app.get_webview_window("search") {
        info!("Search window found!");
        let is_visible = search_window.is_visible().unwrap_or(false);
        info!("Search window current visibility: {}", is_visible);
        
        if is_visible {
            info!("Hiding search window...");
            search_window.hide().map_err(|e| e.to_string())?;
            info!("Search launcher hidden");
        } else {
            info!("Showing search window...");
            // Improved multi-monitor positioning
            info!("Centering window on active screen...");
            center_window_on_active_screen(&search_window).await?;
            
            // Critical: Set always on top FIRST
            info!("Setting always on top...");
            search_window.set_always_on_top(true).map_err(|e| e.to_string())?;
            
            // Show window sequence
            info!("Showing window...");
            search_window.show().map_err(|e| e.to_string())?;
            info!("Unminimizing window...");
            search_window.unminimize().map_err(|e| e.to_string())?;
            info!("Setting focus...");
            search_window.set_focus().map_err(|e| e.to_string())?;
            
            info!("Search launcher shown and properly positioned");
        }
    } else {
        error!("Search window not found!");
        *state.shortcut_processing.lock().unwrap() = false;
        return Err("Search window not found".to_string());
    }
    
    // Reset processing flag with a longer delay to prevent rapid double-triggers
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        let state: State<AppState> = app_clone.state();
        *state.shortcut_processing.lock().unwrap() = false;
        info!("Shortcut processing flag reset");
    });
    
    Ok(())
}

#[tauri::command]
async fn show_main_window<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    if let Some(main_window) = app.get_webview_window("main") {
        main_window.show().map_err(|e| e.to_string())?;
        main_window.set_focus().map_err(|e| e.to_string())?;
        info!("Main window shown");
    }
    Ok(())
}





#[tauri::command]
async fn execute_module_function(module_id: String, function_id: String) -> Result<String, String> {
    api::modules::execute_module_function_handler(module_id, function_id).await
}

// Improved multi-monitor positioning
async fn center_window_on_active_screen<R: Runtime>(window: &tauri::WebviewWindow<R>) -> Result<(), String> {
    // Get cursor position to determine active monitor
    if let Ok(cursor_pos) = window.cursor_position() {
        // Get available monitors
        if let Ok(monitors) = window.available_monitors() {
            // Find monitor containing cursor
            for monitor in monitors {
                let pos = monitor.position();
                let size = monitor.size();
                
                if cursor_pos.x >= pos.x as f64 
                    && cursor_pos.x <= (pos.x + size.width as i32) as f64
                    && cursor_pos.y >= pos.y as f64 
                    && cursor_pos.y <= (pos.y + size.height as i32) as f64 {
                    
                    // Get window size
                    let window_size = window.outer_size().map_err(|e| e.to_string())?;
                    
                    // Calculate center position (slightly above center like Spotlight)
                    let center_x = pos.x + (size.width as i32 - window_size.width as i32) / 2;
                    let center_y = pos.y + (size.height as i32 - window_size.height as i32) / 3; // 1/3 from top
                    
                    window.set_position(tauri::Position::Physical(
                        tauri::PhysicalPosition { x: center_x, y: center_y }
                    )).map_err(|e| e.to_string())?;
                    
                    info!("Window positioned on active monitor at ({}, {})", center_x, center_y);
                    return Ok(());
                }
            }
        }
    }
    
    // Fallback to simple center
    window.center().map_err(|e| e.to_string())?;
    info!("Window centered using fallback method");
    Ok(())
}

// macOS specific behavior setup
#[cfg(target_os = "macos")]
fn setup_macos_behavior<R: Runtime>(window: &tauri::WebviewWindow<R>) -> Result<(), String> {
    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};
    use cocoa::base::nil;
    use objc::{msg_send, sel, sel_impl};
    
    unsafe {
        let app: cocoa::base::id = NSApp();
        // Activate app ignoring other apps (brings to front)
        let _: () = msg_send![app, activateIgnoringOtherApps: cocoa::base::YES];
        // Set activation policy to regular (allows focus)
        let _: () = msg_send![app, setActivationPolicy: NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular];
    }
    
    info!("macOS specific behavior configured");
    Ok(())
}

// Force window to front more aggressively
#[cfg(target_os = "macos")]
fn force_window_to_front<R: Runtime>(window: &tauri::WebviewWindow<R>) -> Result<(), String> {
    // Simplified approach - just use Tauri's built-in methods for now
    // The aggressive NSWindow operations were causing crashes
    window.set_always_on_top(true).map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    
    info!("Window forced to front using simplified approach");
    Ok(())
}


// --- Atalho Global Corrigido para v2 ---
fn setup_global_shortcuts<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up global shortcut for CommandOrControl+Space");
    let manager = app.global_shortcut();
    
    // Clear any existing shortcut first
    let _ = manager.unregister("CmdOrCtrl+Space");
    
    // Add small delay before registering to ensure clean state
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    manager.on_shortcut("CmdOrCtrl+Space", move |app_handle, _shortcut, event| {
        // Only trigger on key press, not release
        if event.state == ShortcutState::Pressed {
            info!("Global shortcut triggered - toggling launcher");
            let app = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                let _ = toggle_search_launcher(app).await;
            });
        }
    })?;

    info!("Global shortcut registered successfully");
    Ok(())
}

// --- Função `main` Corrigida para v2 ---
fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        // Register global-shortcut plugin via builder
        .plugin(GlobalShortcutBuilder::new().build())
        .on_tray_icon_event(|app, event| {
            if let TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                if let Some(main_window) = app.get_webview_window("main") {
                    if main_window.is_visible().unwrap_or(false) {
                        let _ = main_window.hide();
                    } else {
                        let _ = main_window.show();
                        let _ = main_window.set_focus();
                    }
                }
            }
        })
        .on_window_event(|_window, event| {
            match event {
                tauri::WindowEvent::Focused(_focused) => {
                    // Temporarily disabled auto-hide to avoid conflicts during testing
                    // Will re-enable after shortcut issues are resolved
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            search_files,
            search_apps,
            get_module_data,
            ai_query,
            set_selected_module,
            get_selected_module,
            toggle_search_launcher,
            show_main_window,
            execute_module_function,
            commands::settings::load_settings_cmd,
            commands::settings::save_settings_cmd,
            commands::settings::reset_settings_cmd,
            commands::settings::update_autostart_cmd,
            commands::auth::login,
            commands::auth::complete_first_access,
            commands::icons::get_file_icon,
            commands::icons::get_file_icons_batch,
            commands::icons::clear_icon_cache,
            commands::icons::get_cache_stats
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            let handle_clone = handle.clone();

            tauri::async_runtime::spawn(async move {
                logger::init().expect("Falha ao inicializar logger");
                if let Err(e) = load_dotenv() {
                    error!("Fatal error loading .env: {}", e);
                    std::process::exit(1);
                }
                info!("Logger and .env loaded.");

                let search_engine = SearchEngine::new().await.expect("Failed to initialize search engine");
                handle_clone.manage(search_engine);
                info!("Search engine initialized and managed.");
            });

            // List all available windows to debug
            info!("Available windows:");
            for window in app.webview_windows() {
                info!("  - Window: {} (label: {})", window.0, window.0);
            }

            if let Err(e) = setup_global_shortcuts(app.handle()) {
                error!("Failed to set up global shortcuts: {}", e);
            }
            
            // Check if search window exists
            if let Some(search_window) = app.get_webview_window("search") {
                info!("Search window found during setup: {:?}", search_window.label());
                // Ensure the search window is hidden initially and independent
                search_window.hide().unwrap_or_default();
                search_window.set_always_on_top(true).unwrap_or_default();
                info!("Search window configured as independent floating window");
            } else {
                error!("Search window NOT found during setup!");
                
                // Create the search window explicitly if it doesn't exist
                use tauri::{WebviewWindowBuilder, WebviewUrl};
                let search_window = WebviewWindowBuilder::new(
                    app,
                    "search",
                    WebviewUrl::App("index.html".into())
                )
                .title("")
                .inner_size(600.0, 400.0)
                .resizable(false)
                .decorations(false)
                .transparent(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .visible(false)
                .center()
                .build();
                
                match search_window {
                    Ok(window) => {
                        info!("Search window created successfully as independent window");
                        window.hide().unwrap_or_default();
                    },
                    Err(e) => {
                        error!("Failed to create search window: {}", e);
                    }
                }
            }
            
            // Check if user has selected a module (indicates setup is done)
            let state: State<AppState> = handle.state();
            let has_selected_module = state.selected_module.lock().unwrap().is_some();
            
            if !has_selected_module {
                // Show main window for first-time setup or when no module selected
                if let Some(main_window) = app.get_webview_window("main") {
                    main_window.show()?;
                    main_window.set_focus()?;
                    info!("R5 Flowlight: Showing main window for setup");
                }
            } else {
                info!("R5 Flowlight: Started in launcher mode");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
