// Shared types across all components

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub version: String,
    pub settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub current_module: Option<String>,
    pub modules: Vec<Module>,
    pub ui_settings: UiSettings,
    pub search_settings: SearchSettings,
    pub system_settings: SystemSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub theme: String,
    pub window_width: f64,
    pub window_height: f64,
    pub auto_hide_delay: u64,
    pub animations_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSettings {
    pub max_results: usize,
    pub search_timeout_ms: u64,
    pub fuzzy_search_enabled: bool,
    pub include_system_apps: bool,
    pub include_files: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettings {
    pub global_shortcut: String,
    pub start_on_boot: bool,
    pub minimize_to_tray: bool,
    pub logging_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            current_module: None,
            modules: Vec::new(),
            ui_settings: UiSettings::default(),
            search_settings: SearchSettings::default(),
            system_settings: SystemSettings::default(),
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            window_width: 600.0,
            window_height: 400.0,
            auto_hide_delay: 150,
            animations_enabled: true,
        }
    }
}

impl Default for SearchSettings {
    fn default() -> Self {
        Self {
            max_results: 20,
            search_timeout_ms: 5000,
            fuzzy_search_enabled: true,
            include_system_apps: true,
            include_files: true,
        }
    }
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            global_shortcut: "CmdOrCtrl+Space".to_string(),
            start_on_boot: false,
            minimize_to_tray: true,
            logging_level: "info".to_string(),
        }
    }
}