// em /Users/rafaelreis/R5 Flowlight/src-tauri/src/commands/settings/mod.rs
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use serde::{Deserialize, Serialize};
use tauri::{Runtime, path::PathResolver};
use log::{info, error, debug};
use std::fs::File;
use std::io::Write;

// Re-export the commands
pub use commands::*;

// Module for Tauri commands
pub mod commands;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub search: SearchSettings,
    pub appearance: AppearanceSettings,
    pub hotkeys: HotkeySettings,
    pub ai: AISettings,
    pub privacy: PrivacySettings,
    pub performance: PerformanceSettings,
    pub integrations: IntegrationSettings,
    pub notifications: NotificationSettings,
    #[serde(rename = "folderIcons")]
    pub folder_icons: FolderIconsSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralSettings {
    #[serde(rename = "startWithSystem")]
    pub start_with_system: bool,
    #[serde(rename = "showInTaskbar")]
    pub show_in_taskbar: bool,
    #[serde(rename = "closeToTray")]
    pub close_to_tray: bool,
    pub language: String,
    #[serde(rename = "dateFormat")]
    pub date_format: String,
    #[serde(rename = "numberFormat")]
    pub number_format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchSettings {
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    #[serde(rename = "searchDelay")]
    pub search_delay: i32,
    #[serde(rename = "fuzzySearch")]
    pub fuzzy_search: bool,
    #[serde(rename = "includedPaths")]
    pub included_paths: Vec<String>,
    #[serde(rename = "excludedPatterns")]
    pub excluded_patterns: Vec<String>,
    #[serde(rename = "contentTypes")]
    pub content_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppearanceSettings {
    pub theme: String,
    #[serde(rename = "fontSize")]
    pub font_size: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HotkeySettings {
    #[serde(rename = "openApp")]
    pub open_app: String,
    pub search: String,
    #[serde(rename = "newNote")]
    pub new_note: String,
    pub settings: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AISettings {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub model: String,
    pub temperature: f64,
    #[serde(rename = "contextMemory")]
    pub context_memory: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivacySettings {
    pub telemetry: bool,
    #[serde(rename = "crashReports")]
    pub crash_reports: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceSettings {
    #[serde(rename = "hardwareAcceleration")]
    pub hardware_acceleration: bool,
    #[serde(rename = "backgroundThrottling")]
    pub background_throttling: bool,
    #[serde(rename = "maxConcurrent")]
    pub max_concurrent: i32,
    #[serde(rename = "cacheSize")]
    pub cache_size: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntegrationSettings {
    pub github: bool,
    pub slack: bool,
    pub google: bool,
    pub notion: bool,
    pub figma: bool,
    pub webhooks: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationSettings {
    pub enabled: bool,
    pub sounds: bool,
    pub position: String,
    pub duration: i32,
    #[serde(rename = "showProgress")]
    pub show_progress: bool,
    #[serde(rename = "useNative")]
    pub use_native: bool,
    #[serde(rename = "showWhenFocused")]
    pub show_when_focused: bool,
    pub types: NotificationTypes,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NotificationTypes {
    pub info: bool,
    pub success: bool,
    pub warning: bool,
    pub error: bool,
    pub update: bool,
    pub reminder: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderIconsSettings {
    pub enabled: bool,
    pub size: String,
    pub theme: String,
    #[serde(rename = "customIcons")]
    pub custom_icons: std::collections::HashMap<String, String>,
    #[serde(rename = "showInSidebar")]
    pub show_in_sidebar: bool,
    #[serde(rename = "showInFileExplorer")]
    pub show_in_file_explorer: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings {
                start_with_system: true,
                show_in_taskbar: true,
                close_to_tray: true,
                language: "pt-BR".to_string(),
                date_format: "dd/MM/yyyy".to_string(),
                number_format: "pt-BR".to_string(),
            },
            search: SearchSettings {
                max_results: 50,
                search_delay: 300,
                fuzzy_search: true,
                included_paths: vec![],
                excluded_patterns: vec!["node_modules".to_string(), "*.log".to_string(), "*.tmp".to_string()],
                content_types: vec!["apps".to_string(), "files".to_string(), "system".to_string()],
            },
            appearance: AppearanceSettings {
                theme: "system".to_string(),
                font_size: 14,
            },
            hotkeys: HotkeySettings {
                open_app: "CommandOrControl+Space".to_string(),
                search: "CommandOrControl+K".to_string(),
                new_note: "CommandOrControl+N".to_string(),
                settings: "CommandOrControl+,".to_string(),
            },
            ai: AISettings {
                api_key: "".to_string(),
                model: "gpt-4".to_string(),
                temperature: 0.7,
                context_memory: true,
            },
            privacy: PrivacySettings {
                telemetry: true,
                crash_reports: true,
            },
            performance: PerformanceSettings {
                hardware_acceleration: true,
                background_throttling: true,
                max_concurrent: 4,
                cache_size: 100,
            },
            integrations: IntegrationSettings {
                github: false,
                slack: false,
                google: false,
                notion: false,
                figma: false,
                webhooks: vec![],
            },
            notifications: NotificationSettings {
                enabled: true,
                sounds: true,
                position: "bottom-right".to_string(),
                duration: 5,
                show_progress: true,
                use_native: true,
                show_when_focused: true,
                types: NotificationTypes {
                    info: true,
                    success: true,
                    warning: true,
                    error: true,
                    update: true,
                    reminder: true,
                },
            },
            folder_icons: FolderIconsSettings {
                enabled: true,
                size: "medium".to_string(),
                theme: "default".to_string(),
                custom_icons: std::collections::HashMap::new(),
                show_in_sidebar: true,
                show_in_file_explorer: true,
            },
        }
    }
}

pub fn get_settings_path<R: Runtime>(resolver: &PathResolver<R>) -> Result<PathBuf, String> {
    let dir = resolver
        .app_config_dir()
        .map_err(|e| e.to_string())?;
    Ok(dir.join("settings.json"))
}

pub fn load_settings<R: Runtime>(resolver: &PathResolver<R>) -> Result<AppSettings, String> {
    let settings_path = get_settings_path(resolver)?;
    info!("Carregando configurações de: {:?}", settings_path);

    if !settings_path.exists() {
        info!("Arquivo de configurações não encontrado, criando um novo com valores padrão.");
        let default_settings = AppSettings::default();
        save_settings(resolver, &default_settings)?;
        return Ok(default_settings);
    }

    let settings_content = fs::read_to_string(&settings_path)
        .map_err(|e| format!("Falha ao ler o arquivo de configurações: {}", e))?;

    serde_json::from_str(&settings_content)
        .map_err(|e| format!("Falha ao desserializar as configurações: {}", e))
}

pub fn save_settings<R: Runtime>(resolver: &PathResolver<R>, settings: &AppSettings) -> Result<(), String> {
    let settings_path = get_settings_path(resolver)?;
    let settings_dir = settings_path.parent().ok_or_else(|| "Falha ao obter diretório pai".to_string())?;

    if !settings_dir.exists() {
        fs::create_dir_all(settings_dir).map_err(|e| format!("Falha ao criar diretório: {}", e))?;
    }

    let settings_json = serde_json::to_string_pretty(settings).map_err(|e| format!("Falha ao serializar: {}", e))?;
    fs::write(&settings_path, settings_json).map_err(|e| format!("Falha ao salvar configurações: {}", e))?;

    info!("Configurações salvas com sucesso em {:?}", settings_path);
    Ok(())
}

pub fn reset_settings<R: Runtime>(resolver: &PathResolver<R>) -> Result<(), String> {
    let default_settings = AppSettings::default();
    save_settings(resolver, &default_settings)
}

pub fn update_autostart<R: Runtime>(resolver: &PathResolver<R>, start_with_system: bool) -> Result<(), String> {
    let mut settings = load_settings(resolver)?;
    settings.general.start_with_system = start_with_system;
    save_settings(resolver, &settings)?;

    if let Err(e) = setup_autostart(start_with_system) {
        error!("Falha ao configurar inicialização automática: {}", e);
        return Err(format!("Falha ao configurar inicialização automática: {}", e));
    }

    Ok(())
}

// Helper function to update autostart setting
pub(crate) fn setup_autostart(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    let app_name = "R5 Flowlight";
    let app_path = env::current_exe()?;

    #[cfg(target_os = "windows")]
    {
        use winreg::{
            enums::HKEY_CURRENT_USER,
            RegKey,
        };

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
        let (key, _) = hkcu.create_subkey(path)?;

        if enabled {
            key.set_value(
                app_name,
                &app_path.to_str().unwrap_or(""),
            )?;
        } else {
            let _ = key.delete_value(app_name);
        }
    }

    #[cfg(target_os = "macos")]
    {
        let home_dir = env::var("HOME").unwrap_or_else(|_| "/Users/Shared".to_string());
        let plist_path = format!("{}/Library/LaunchAgents/com.{}.plist", home_dir, app_name.to_lowercase().replace(" ", ""));

        if enabled {
            let plist_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
</dict>
</plist>"#,
                app_name.to_lowercase().replace(" ", ""),
                app_path.to_str().unwrap_or("")
            );

            let mut file = File::create(&plist_path)?;
            file.write_all(plist_content.as_bytes())?;
        } else {
            let _ = std::fs::remove_file(plist_path);
        }
    }

    #[cfg(target_os = "linux")]
    {
        let home_dir = env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        let autostart_dir = format!("{}/.config/autostart", home_dir);
        let desktop_file = format!("{}/{}.desktop", autostart_dir, app_name.to_lowercase().replace(" ", ""));

        if enabled {
            let _ = std::fs::create_dir_all(&autostart_dir);

            let desktop_content = format!(
                "[Desktop Entry]\n\
                Type=Application\n\
                Name={}\n\
                Exec={}",
                app_name,
                app_path.to_str().unwrap_or("")
            );

            let mut file = File::create(&desktop_file)?;
            file.write_all(desktop_content.as_bytes())?;
        } else {
            let _ = std::fs::remove_file(desktop_file);
        }
    }

    info!("Autostart at login {}", if enabled { "enabled" } else { "disabled" });
    Ok(())
}
