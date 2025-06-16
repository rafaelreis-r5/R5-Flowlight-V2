// Daemon State Management

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct DaemonState {
    pub current_module: Option<String>,
    pub shortcut_processing: bool,
    pub last_shortcut_time: u64,
    pub overlay_visible: bool,
    pub search_session_id: Option<String>,
    pub configuration: DaemonConfig,
    pub stats: DaemonStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    pub debounce_delay_ms: u64,
    pub auto_hide_delay_ms: u64,
    pub max_search_results: usize,
    pub search_timeout_ms: u64,
    pub module_configs: HashMap<String, ModuleConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub enabled: bool,
    pub priority: i32,
    pub custom_settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct DaemonStats {
    pub shortcuts_triggered: u64,
    pub searches_performed: u64,
    pub uptime_seconds: u64,
    pub last_activity: u64,
    pub memory_usage_kb: u64,
}

impl DaemonState {
    pub fn new() -> Self {
        Self {
            current_module: None,
            shortcut_processing: false,
            last_shortcut_time: 0,
            overlay_visible: false,
            search_session_id: None,
            configuration: DaemonConfig::default(),
            stats: DaemonStats::new(),
        }
    }
    
    pub fn set_current_module(&mut self, module_id: String) {
        self.current_module = Some(module_id);
        self.update_activity();
    }
    
    pub fn clear_current_module(&mut self) {
        self.current_module = None;
        self.update_activity();
    }
    
    pub fn is_processing_shortcut(&self) -> bool {
        if !self.shortcut_processing {
            return false;
        }
        
        // Check if enough time has passed since last shortcut
        let now = current_timestamp();
        let elapsed = now.saturating_sub(self.last_shortcut_time);
        
        elapsed < self.configuration.debounce_delay_ms
    }
    
    pub fn set_processing_shortcut(&mut self, processing: bool) {
        self.shortcut_processing = processing;
        if processing {
            self.last_shortcut_time = current_timestamp();
            self.stats.shortcuts_triggered += 1;
        }
        self.update_activity();
    }
    
    pub fn set_overlay_visible(&mut self, visible: bool) {
        self.overlay_visible = visible;
        self.update_activity();
    }
    
    pub fn start_search_session(&mut self) -> String {
        let session_id = uuid::Uuid::new_v4().to_string();
        self.search_session_id = Some(session_id.clone());
        self.stats.searches_performed += 1;
        self.update_activity();
        session_id
    }
    
    pub fn end_search_session(&mut self) {
        self.search_session_id = None;
        self.update_activity();
    }
    
    pub fn get_module_config(&self, module_id: &str) -> Option<&ModuleConfig> {
        self.configuration.module_configs.get(module_id)
    }
    
    pub fn set_module_config(&mut self, module_id: String, config: ModuleConfig) {
        self.configuration.module_configs.insert(module_id, config);
        self.update_activity();
    }
    
    pub fn update_stats(&mut self, memory_usage_kb: u64) {
        self.stats.memory_usage_kb = memory_usage_kb;
        self.update_activity();
    }
    
    fn update_activity(&mut self) {
        self.stats.last_activity = current_timestamp();
    }
    
    pub fn get_uptime(&self) -> u64 {
        current_timestamp().saturating_sub(self.stats.last_activity)
    }
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            debounce_delay_ms: 300,
            auto_hide_delay_ms: 150,
            max_search_results: 20,
            search_timeout_ms: 5000,
            module_configs: HashMap::new(),
        }
    }
}

impl DaemonStats {
    pub fn new() -> Self {
        let now = current_timestamp();
        Self {
            shortcuts_triggered: 0,
            searches_performed: 0,
            uptime_seconds: 0,
            last_activity: now,
            memory_usage_kb: 0,
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemon_state_creation() {
        let state = DaemonState::new();
        assert!(state.current_module.is_none());
        assert!(!state.shortcut_processing);
        assert!(!state.overlay_visible);
    }

    #[test]
    fn test_shortcut_debouncing() {
        let mut state = DaemonState::new();
        
        // First shortcut should not be processing
        assert!(!state.is_processing_shortcut());
        
        // Set processing
        state.set_processing_shortcut(true);
        assert!(state.is_processing_shortcut());
        
        // After clearing processing flag
        state.set_processing_shortcut(false);
        assert!(!state.is_processing_shortcut());
    }

    #[test]
    fn test_search_session() {
        let mut state = DaemonState::new();
        
        let session_id = state.start_search_session();
        assert!(state.search_session_id.is_some());
        assert_eq!(state.search_session_id.as_ref().unwrap(), &session_id);
        assert_eq!(state.stats.searches_performed, 1);
        
        state.end_search_session();
        assert!(state.search_session_id.is_none());
    }

    #[test]
    fn test_module_management() {
        let mut state = DaemonState::new();
        
        state.set_current_module("test_module".to_string());
        assert_eq!(state.current_module.as_ref().unwrap(), "test_module");
        
        state.clear_current_module();
        assert!(state.current_module.is_none());
    }
}