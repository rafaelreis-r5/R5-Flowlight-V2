// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod api;
mod modules;
mod ai;
mod utils;

// Authentication server
mod auth_server;

use tauri::{AppHandle, Manager, WindowBuilder, WindowUrl};
use log::{info, error};
use std::sync::Mutex;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

use crate::core::search_engine::SearchEngine;
use crate::utils::config::AppConfig;
use crate::utils::logger;

// Global state for selected module
struct AppState {
    selected_module: Mutex<Option<String>>,
}

// Tauri commands
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
async fn set_selected_module(
    state: tauri::State<'_, AppState>,
    module_id: String,
) -> Result<(), String> {
    let mut selected_module = state.selected_module.lock().unwrap();
    *selected_module = Some(module_id.clone());
    info!("Selected module set to: {}", module_id);
    Ok(())
}

#[tauri::command]
async fn get_selected_module(
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let selected_module = state.selected_module.lock().unwrap();
    match selected_module.as_ref() {
        Some(module) => Ok(module.clone()),
        None => Ok("general".to_string()),
    }
}

#[tauri::command]
async fn setup_search_window(
    app: AppHandle,
    module_id: String,
) -> Result<(), String> {
    info!("Setting up search window for module: {}", module_id);
    
    // Create search window if it doesn't exist
    if app.get_window("search").is_none() {
        let search_window = WindowBuilder::new(
            &app,
            "search",
            WindowUrl::App("search.html".into())
        )
        .title("")
        .inner_size(700.0, 60.0)
        .resizable(false)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .center()
        .visible(false)
        .build()
        .map_err(|e| format!("Failed to create search window: {}", e))?;

        info!("Search window created successfully");
    }

    Ok(())
}

#[tauri::command]
async fn show_search_window(app: AppHandle) -> Result<(), String> {
    if let Some(search_window) = app.get_window("search") {
        search_window.show().map_err(|e| format!("Failed to show search window: {}", e))?;
        search_window.set_focus().map_err(|e| format!("Failed to focus search window: {}", e))?;
        info!("Search window shown and focused");
    } else {
        return Err("Search window not found".to_string());
    }
    Ok(())
}

#[tauri::command]
async fn hide_search_window(app: AppHandle) -> Result<(), String> {
    if let Some(search_window) = app.get_window("search") {
        search_window.hide().map_err(|e| format!("Failed to hide search window: {}", e))?;
        info!("Search window hidden");
    }
    Ok(())
}

#[tauri::command]
async fn hide_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(main_window) = app.get_window("main") {
        main_window.hide().map_err(|e| format!("Failed to hide main window: {}", e))?;
        info!("Main window hidden");
    }
    Ok(())
}

#[tauri::command]
async fn show_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(main_window) = app.get_window("main") {
        main_window.show().map_err(|e| format!("Failed to show main window: {}", e))?;
        main_window.set_focus().map_err(|e| format!("Failed to focus main window: {}", e))?;
        info!("Main window shown and focused");
    }
    Ok(())
}

#[tauri::command]
async fn resize_search_window(app: AppHandle, height: f64) -> Result<(), String> {
    if let Some(search_window) = app.get_window("search") {
        search_window.set_size(tauri::LogicalSize::new(700.0, height))
            .map_err(|e| format!("Failed to resize search window: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn execute_module_function(
    module_id: String,
    function_id: String
) -> Result<String, String> {
    info!("Executing function {} for module {}", function_id, module_id);
    api::modules::execute_module_function_handler(module_id, function_id).await
}

// TODO: Implementar setup_global_shortcuts quando plugins estiverem disponíveis
fn setup_global_shortcuts(_app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    info!("Global shortcuts setup - será implementado com plugins");
    // TODO: Registrar Cmd+Space / Ctrl+Space para mostrar search window
    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize logger
    logger::init();
    info!("Starting R5 Flowlight...");

    // Load configuration
    let config = AppConfig::load().unwrap_or_else(|e| {
        error!("Failed to load config: {}. Using default config.", e);
        AppConfig::default()
    });

    // Start authentication server in a separate task
    let auth_server_handle = tokio::spawn(async move {
        if let Err(e) = auth_server::start_auth_server(3030).await {
            error!("Authentication server error: {}", e);
        }
    });
    
    // Initialize search engine
    let search_engine = SearchEngine::new(&config.search_index_path)
        .expect("Failed to initialize search engine");
    
    info!("Search engine initialized at path: {}", config.search_index_path);

    // Create Tauri app
    tauri::Builder::default()
        .manage(AppState {
            selected_module: Mutex::new(None),
        })
        .manage(search_engine)
        .manage(TokioMutex::new(())) // For async state sharing if needed
        .invoke_handler(tauri::generate_handler![
            search_files,
            search_apps,
            get_module_data,
            ai_query,
            set_selected_module,
            get_selected_module,
            setup_search_window,
            show_search_window,
            hide_search_window,
            hide_main_window,
            show_main_window,
            resize_search_window,
            execute_module_function
        ])
        .setup(|app| {
            // Set up global shortcuts
            if let Err(e) = setup_global_shortcuts(app) {
                error!("Failed to set up global shortcuts: {}", e);
            }

            // Create main window
            let main_window = WindowBuilder::new(
                app,
                "main",
                WindowUrl::App("index.html".into())
            )
            .title("R5 Flowlight")
            .inner_size(800.0, 600.0)
            .resizable(true)
            .fullscreen(false)
            .build()?;

            // Store window reference
            let window = app.get_window("main").expect("Failed to get main window");
            
            // Set up event listeners
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { .. } = event {
                    let _ = window_clone.hide();
                }
            });

            info!("Main window configured and displayed");
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Wait for the auth server to finish (which it won't unless there's an error)
    let _ = auth_server.await;
}