// Module trait definitions for R5 Flowlight

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub icon: Option<String>,
    pub action_type: String,
    pub score: f32,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: String,
    pub module_filter: Option<String>,
    pub max_results: usize,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub enabled: bool,
    pub keywords: Vec<String>,
}

#[async_trait]
pub trait SearchModule: Send + Sync {
    /// Get module information
    fn info(&self) -> ModuleInfo;
    
    /// Initialize the module with configuration
    async fn initialize(&mut self, config: HashMap<String, serde_json::Value>) -> anyhow::Result<()>;
    
    /// Perform a search with the given query
    async fn search(&self, query: &SearchQuery) -> anyhow::Result<Vec<SearchResult>>;
    
    /// Execute an action for a specific result
    async fn execute_action(&self, result_id: &str, action_type: &str) -> anyhow::Result<()>;
    
    /// Check if the module is healthy and working
    async fn health_check(&self) -> anyhow::Result<bool>;
    
    /// Get module settings schema
    fn get_settings_schema(&self) -> HashMap<String, serde_json::Value>;
    
    /// Update module settings
    async fn update_settings(&mut self, settings: HashMap<String, serde_json::Value>) -> anyhow::Result<()>;
    
    /// Cleanup module resources
    async fn cleanup(&mut self) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct ModuleSearchContext {
    pub query: SearchQuery,
    pub user_preferences: HashMap<String, String>,
    pub system_info: SystemInfo,
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub platform: String,
    pub current_app: Option<String>,
    pub screen_resolution: (u32, u32),
    pub current_workspace: Option<String>,
}