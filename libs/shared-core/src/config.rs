// Enhanced Configuration management for R5 Flowlight

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use log::{info, error, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R5Config {
    pub app: AppConfig,
    pub modules: HashMap<String, ModuleConfig>,
    pub shortcuts: ShortcutConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub enabled: bool,
    pub priority: i32,
    pub settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub global_shortcut: String,
    pub show_overlay: String,
    pub hide_overlay: String,
    pub next_result: String,
    pub prev_result: String,
    pub execute_result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub overlay_width: f64,
    pub overlay_height: f64,
    pub max_results: usize,
    pub animation_speed: f64,
    pub auto_hide_delay: u64,
}

impl Default for R5Config {
    fn default() -> Self {
        let mut modules = HashMap::new();
        
        // Default daily module configuration
        modules.insert("daily".to_string(), ModuleConfig {
            enabled: true,
            priority: 1,
            settings: {
                let mut settings = HashMap::new();
                settings.insert("cache_update_interval".to_string(), serde_json::json!(30));
                settings.insert("show_system_actions".to_string(), serde_json::json!(true));
                settings.insert("time_format_24h".to_string(), serde_json::json!(true));
                settings
            },
        });
        
        Self {
            app: AppConfig::default(),
            modules,
            shortcuts: ShortcutConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            global_shortcut: "CmdOrCtrl+Space".to_string(),
            show_overlay: "CmdOrCtrl+Space".to_string(),
            hide_overlay: "Escape".to_string(),
            next_result: "ArrowDown".to_string(),
            prev_result: "ArrowUp".to_string(),
            execute_result: "Enter".to_string(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            overlay_width: 600.0,
            overlay_height: 400.0,
            max_results: 10,
            animation_speed: 0.2,
            auto_hide_delay: 150,
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: R5Config,
}

impl ConfigManager {
    pub fn new() -> anyhow::Result<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_or_create_config(&config_path)?;
        
        Ok(Self {
            config_path,
            config,
        })
    }
    
    fn get_config_path() -> anyhow::Result<PathBuf> {
        let mut config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        
        config_dir.push("R5Flowlight");
        
        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
            info!("📁 Created config directory: {:?}", config_dir);
        }
        
        config_dir.push("config.json");
        Ok(config_dir)
    }
    
    fn load_or_create_config(config_path: &PathBuf) -> anyhow::Result<R5Config> {
        if config_path.exists() {
            info!("📖 Loading configuration from: {:?}", config_path);
            let content = std::fs::read_to_string(config_path)?;
            
            match serde_json::from_str::<R5Config>(&content) {
                Ok(config) => {
                    info!("✅ Configuration loaded successfully");
                    Ok(config)
                }
                Err(e) => {
                    warn!("⚠️  Failed to parse config file: {}. Creating default config.", e);
                    let default_config = R5Config::default();
                    Self::save_config(config_path, &default_config)?;
                    Ok(default_config)
                }
            }
        } else {
            info!("🆕 Creating default configuration at: {:?}", config_path);
            let default_config = R5Config::default();
            Self::save_config(config_path, &default_config)?;
            Ok(default_config)
        }
    }
    
    fn save_config(config_path: &PathBuf, config: &R5Config) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(config)?;
        std::fs::write(config_path, content)?;
        info!("💾 Configuration saved to: {:?}", config_path);
        Ok(())
    }
    
    pub fn get_config(&self) -> &R5Config {
        &self.config
    }
    
    pub fn get_config_mut(&mut self) -> &mut R5Config {
        &mut self.config
    }
    
    pub fn save(&self) -> anyhow::Result<()> {
        Self::save_config(&self.config_path, &self.config)
    }
    
    pub fn reload(&mut self) -> anyhow::Result<()> {
        self.config = Self::load_or_create_config(&self.config_path)?;
        Ok(())
    }
    
    // Convenience methods for common operations
    pub fn get_global_shortcut(&self) -> &str {
        &self.config.shortcuts.global_shortcut
    }
    
    pub fn set_global_shortcut(&mut self, shortcut: String) -> anyhow::Result<()> {
        self.config.shortcuts.global_shortcut = shortcut;
        self.save()
    }
    
    pub fn get_module_config(&self, module_id: &str) -> Option<&ModuleConfig> {
        self.config.modules.get(module_id)
    }
    
    pub fn set_module_enabled(&mut self, module_id: &str, enabled: bool) -> anyhow::Result<()> {
        if let Some(module_config) = self.config.modules.get_mut(module_id) {
            module_config.enabled = enabled;
            self.save()?;
            info!("🔧 Module '{}' {}", module_id, if enabled { "enabled" } else { "disabled" });
        } else {
            error!("❌ Module '{}' not found in configuration", module_id);
        }
        Ok(())
    }
    
    pub fn add_module_config(&mut self, module_id: String, config: ModuleConfig) -> anyhow::Result<()> {
        self.config.modules.insert(module_id.clone(), config);
        self.save()?;
        info!("➕ Added configuration for module '{}'", module_id);
        Ok(())
    }
    
    pub fn get_ui_config(&self) -> &UiConfig {
        &self.config.ui
    }
    
    pub fn set_theme(&mut self, theme: String) -> anyhow::Result<()> {
        self.config.ui.theme = theme;
        self.save()
    }
    
    pub fn set_overlay_size(&mut self, width: f64, height: f64) -> anyhow::Result<()> {
        self.config.ui.overlay_width = width;
        self.config.ui.overlay_height = height;
        self.save()
    }
    
    pub fn export_config(&self, path: &PathBuf) -> anyhow::Result<()> {
        Self::save_config(path, &self.config)?;
        info!("📤 Configuration exported to: {:?}", path);
        Ok(())
    }
    
    pub fn import_config(&mut self, path: &PathBuf) -> anyhow::Result<()> {
        if !path.exists() {
            return Err(anyhow::anyhow!("Config file does not exist: {:?}", path));
        }
        
        let content = std::fs::read_to_string(path)?;
        self.config = serde_json::from_str(&content)?;
        self.save()?;
        
        info!("📥 Configuration imported from: {:?}", path);
        Ok(())
    }
    
    // Legacy compatibility methods
    pub fn load_app_config(&self) -> anyhow::Result<AppConfig> {
        Ok(self.config.app.clone())
    }
    
    pub fn save_app_config(&self, _app_config: &AppConfig) -> anyhow::Result<()> {
        // This method exists for backward compatibility
        // In the new system, use save() instead
        self.save()
    }
    
    pub fn get_config_file_path(&self) -> &PathBuf {
        &self.config_path
    }
}