// R5 Flowlight Main Application
// Login, Dashboard, Module Management

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{info, error, warn, debug};
use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::{AppHandle, Manager, WebviewWindow, Emitter};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};

use ipc_communication::{IPCClient, IPCMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub version: String,
    pub enabled: bool,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub logged_in: bool,
    pub current_user: Option<User>,
    pub active_module: Option<String>,
    pub available_modules: Vec<Module>,
    pub daemon_connected: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            logged_in: false,
            current_user: None,
            active_module: None,
            available_modules: vec![
                Module {
                    id: "daily".to_string(),
                    name: "Daily Tasks".to_string(),
                    description: "Manage your daily tasks and productivity".to_string(),
                    icon: "📋".to_string(),
                    version: "1.0.0".to_string(),
                    enabled: true,
                    active: false,
                },
                Module {
                    id: "files".to_string(),
                    name: "File Search".to_string(),
                    description: "Find files and documents quickly".to_string(),
                    icon: "📁".to_string(),
                    version: "1.0.0".to_string(),
                    enabled: true,
                    active: false,
                },
                Module {
                    id: "web".to_string(),
                    name: "Web Search".to_string(),
                    description: "Search the web and bookmarks".to_string(),
                    icon: "🌐".to_string(),
                    version: "1.0.0".to_string(),
                    enabled: true,
                    active: false,
                },
                Module {
                    id: "apps".to_string(),
                    name: "Applications".to_string(),
                    description: "Launch applications and tools".to_string(),
                    icon: "🚀".to_string(),
                    version: "1.0.0".to_string(),
                    enabled: true,
                    active: false,
                },
                Module {
                    id: "calculator".to_string(),
                    name: "Calculator".to_string(),
                    description: "Perform calculations and conversions".to_string(),
                    icon: "🧮".to_string(),
                    version: "1.0.0".to_string(),
                    enabled: true,
                    active: false,
                },
            ],
            daemon_connected: false,
        }
    }
}

#[tauri::command]
async fn login(
    username: String,
    password: String,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<User, String> {
    info!("🔐 Login attempt for user: {}", username);
    
    // Simple authentication - in production, this would connect to a real auth service
    let password_hash = format!("{:x}", Sha256::digest(password.as_bytes()));
    
    // Demo credentials: admin/admin or user/password
    let valid_login = match username.as_str() {
        "admin" => password == "admin",
        "user" => password == "password",
        "demo" => password == "demo",
        _ => false,
    };
    
    if valid_login {
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: username.clone(),
            email: format!("{}@r5flowlight.com", username),
            created_at: Utc::now(),
            last_login: Some(Utc::now()),
        };
        
        let mut app_state = state.write().await;
        app_state.logged_in = true;
        app_state.current_user = Some(user.clone());
        
        info!("✅ Login successful for user: {}", username);
        Ok(user)
    } else {
        warn!("❌ Login failed for user: {}", username);
        Err("Invalid credentials".to_string())
    }
}

#[tauri::command]
async fn logout(
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
    ipc_client: tauri::State<'_, Arc<RwLock<Option<IPCClient>>>>,
) -> Result<(), String> {
    info!("🚪 User logging out");
    
    // Deactivate current module in daemon
    {
        let app_state = state.read().await;
        if let Some(module_id) = &app_state.active_module {
            let mut client = ipc_client.write().await;
            if let Some(client) = client.as_mut() {
                let message = IPCMessage::ModuleChanged { 
                    module_id: "none".to_string() 
                };
                let _ = client.send(message).await;
                info!("📤 Deactivated module: {}", module_id);
            }
        }
    }
    
    let mut app_state = state.write().await;
    app_state.logged_in = false;
    app_state.current_user = None;
    app_state.active_module = None;
    
    // Deactivate all modules
    for module in &mut app_state.available_modules {
        module.active = false;
    }
    
    info!("✅ Logout completed");
    Ok(())
}

#[tauri::command]
async fn get_app_state(
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<AppState, String> {
    let app_state = state.read().await;
    Ok(app_state.clone())
}

#[tauri::command]
async fn activate_module(
    module_id: String,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
    ipc_client: tauri::State<'_, Arc<RwLock<Option<IPCClient>>>>,
) -> Result<(), String> {
    info!("🎯 Activating module: {}", module_id);
    
    let mut app_state = state.write().await;
    
    // Check if user is logged in
    if !app_state.logged_in {
        return Err("User not logged in".to_string());
    }
    
    // Deactivate all modules first
    for module in &mut app_state.available_modules {
        module.active = false;
    }
    
    // Activate the selected module
    if let Some(module) = app_state.available_modules.iter_mut().find(|m| m.id == module_id) {
        if !module.enabled {
            return Err("Module is not enabled".to_string());
        }
        
        module.active = true;
        app_state.active_module = Some(module_id.clone());
        
        info!("✅ Module activated: {} - {}", module.id, module.name);
        
        // Notify daemon about module change
        let mut client = ipc_client.write().await;
        if let Some(client) = client.as_mut() {
            let message = IPCMessage::ModuleChanged { 
                module_id: module_id.clone() 
            };
            match client.send(message).await {
                Ok(_) => {
                    info!("📤 Notified daemon about module activation: {}", module_id);
                }
                Err(e) => {
                    error!("❌ Failed to notify daemon: {}", e);
                }
            }
        } else {
            warn!("⚠️  Daemon not connected");
        }
        
        Ok(())
    } else {
        Err(format!("Module '{}' not found", module_id))
    }
}

#[tauri::command]
async fn deactivate_module(
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
    ipc_client: tauri::State<'_, Arc<RwLock<Option<IPCClient>>>>,
) -> Result<(), String> {
    info!("🛑 Deactivating current module");
    
    let mut app_state = state.write().await;
    
    if let Some(module_id) = &app_state.active_module {
        // Deactivate all modules
        for module in &mut app_state.available_modules {
            module.active = false;
        }
        
        app_state.active_module = None;
        
        // Notify daemon
        let mut client = ipc_client.write().await;
        if let Some(client) = client.as_mut() {
            let message = IPCMessage::ModuleChanged { 
                module_id: "none".to_string() 
            };
            let _ = client.send(message).await;
            info!("📤 Notified daemon about module deactivation");
        }
        
        info!("✅ Module deactivated: {}", module_id);
    }
    
    Ok(())
}

#[tauri::command]
async fn check_daemon_status(
    ipc_client: tauri::State<'_, Arc<RwLock<Option<IPCClient>>>>,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    let mut client = ipc_client.write().await;
    
    if client.is_none() {
        // Try to connect to daemon
        match IPCClient::new().await {
            Ok(mut new_client) => {
                match new_client.connect().await {
                    Ok(_) => {
                        info!("📡 Connected to daemon");
                        *client = Some(new_client);
                        
                        // Update state
                        let mut app_state = state.write().await;
                        app_state.daemon_connected = true;
                        
                        return Ok(true);
                    }
                    Err(e) => {
                        debug!("Failed to connect to daemon: {}", e);
                        return Ok(false);
                    }
                }
            }
            Err(e) => {
                debug!("Failed to create IPC client: {}", e);
                return Ok(false);
            }
        }
    }
    
    // Check if existing connection is still valid
    if let Some(client) = client.as_mut() {
        match client.send(IPCMessage::Ping).await {
            Ok(_) => {
                debug!("📡 Daemon connection is healthy");
                let mut app_state = state.write().await;
                app_state.daemon_connected = true;
                Ok(true)
            }
            Err(_) => {
                warn!("📡 Daemon connection lost");
                *client = None;
                let mut app_state = state.write().await;
                app_state.daemon_connected = false;
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    info!("🚀 R5 Flowlight Main Application starting...");
    
    // Initialize application state
    let app_state = Arc::new(RwLock::new(AppState::default()));
    let ipc_client: Arc<RwLock<Option<IPCClient>>> = Arc::new(RwLock::new(None));
    
    tauri::Builder::default()
        .manage(app_state.clone())
        .manage(ipc_client.clone())
        .invoke_handler(tauri::generate_handler![
            login,
            logout,
            get_app_state,
            activate_module,
            deactivate_module,
            check_daemon_status
        ])
        .setup(move |app| {
            info!("🪟 Main application window created");
            
            // Try to connect to daemon on startup
            let ipc_client = ipc_client.clone();
            let app_state = app_state.clone();
            tokio::spawn(async move {
                // Wait a bit for daemon to be ready
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                match IPCClient::new().await {
                    Ok(mut client) => {
                        match client.connect().await {
                            Ok(_) => {
                                info!("📡 Initial connection to daemon successful");
                                *ipc_client.write().await = Some(client);
                                
                                let mut state = app_state.write().await;
                                state.daemon_connected = true;
                            }
                            Err(e) => {
                                warn!("⚠️  Could not connect to daemon on startup: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        warn!("⚠️  Could not create IPC client: {}", e);
                    }
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    Ok(())
}