// em /Users/rafaelreis/R5 Flowlight/src-tauri/src/commands/mod.rs
//! Tauri commands module for the Tauri application.
//! This module contains all the command handlers for the application.

pub mod settings;
pub mod auth;
pub mod icons;

// Re-export the command functions from the commands module
pub use settings::commands::{
    load_settings_cmd as load_settings,
    load_settings_cmd,
    save_settings_cmd,
    reset_settings_cmd,
    update_autostart_cmd
};

// Re-export the settings types
pub use settings::{
    AppSettings,
    GeneralSettings,
    SearchSettings,
    AppearanceSettings
};

// Re-export auth commands
pub use auth::{
    login,
    complete_first_access,
    get_user,
    AuthUser,
    AuthResponse
};

// Re-export icons commands
pub use icons::{
    get_file_icon,
    get_file_icons_batch,
    clear_icon_cache,
    get_cache_stats,
    IconResult,
    BatchIconResult
};
