// Base module implementations

use crate::traits::*;
use async_trait::async_trait;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use log::{info, warn, error};
use std::collections::HashMap;

pub struct BaseSearchModule {
    pub info: ModuleInfo,
    pub settings: HashMap<String, serde_json::Value>,
    pub matcher: SkimMatcherV2,
    pub initialized: bool,
}

impl BaseSearchModule {
    pub fn new(info: ModuleInfo) -> Self {
        Self {
            info,
            settings: HashMap::new(),
            matcher: SkimMatcherV2::default(),
            initialized: false,
        }
    }
    
    pub fn fuzzy_search(&self, query: &str, items: &[SearchResult]) -> Vec<SearchResult> {
        let mut scored_results: Vec<(SearchResult, i64)> = items
            .iter()
            .filter_map(|item| {
                // Try matching title first
                if let Some(score) = self.matcher.fuzzy_match(&item.title, query) {
                    return Some((item.clone(), score));
                }
                
                // Then try description
                if let Some(score) = self.matcher.fuzzy_match(&item.description, query) {
                    return Some((item.clone(), score / 2)); // Lower priority for description matches
                }
                
                None
            })
            .collect();
        
        // Sort by score (descending)
        scored_results.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Update scores and return results
        scored_results
            .into_iter()
            .enumerate()
            .map(|(index, (mut result, score))| {
                // Normalize score to 0.0-1.0 range
                result.score = (score as f32 / 1000.0).min(1.0).max(0.0);
                
                // Apply small penalty for position
                result.score *= 1.0 - (index as f32 * 0.01);
                
                result
            })
            .collect()
    }
    
    pub fn exact_search(&self, query: &str, items: &[SearchResult]) -> Vec<SearchResult> {
        let query_lower = query.to_lowercase();
        
        items
            .iter()
            .filter(|item| {
                item.title.to_lowercase().contains(&query_lower) ||
                item.description.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect()
    }
}

#[async_trait]
impl SearchModule for BaseSearchModule {
    fn info(&self) -> ModuleInfo {
        self.info.clone()
    }
    
    async fn initialize(&mut self, config: HashMap<String, serde_json::Value>) -> anyhow::Result<()> {
        self.settings = config;
        self.initialized = true;
        info!("✅ Base module '{}' initialized", self.info.name);
        Ok(())
    }
    
    async fn search(&self, _query: &SearchQuery) -> anyhow::Result<Vec<SearchResult>> {
        // Base implementation returns empty results
        // Concrete modules should override this
        Ok(Vec::new())
    }
    
    async fn execute_action(&self, result_id: &str, action_type: &str) -> anyhow::Result<()> {
        warn!("🚫 Base module cannot execute action '{}' for result '{}'", action_type, result_id);
        Err(anyhow::anyhow!("Base module does not support action execution"))
    }
    
    async fn health_check(&self) -> anyhow::Result<bool> {
        Ok(self.initialized)
    }
    
    fn get_settings_schema(&self) -> HashMap<String, serde_json::Value> {
        let mut schema = HashMap::new();
        schema.insert("enabled".to_string(), serde_json::json!({
            "type": "boolean",
            "default": true,
            "description": "Enable this module"
        }));
        schema
    }
    
    async fn update_settings(&mut self, settings: HashMap<String, serde_json::Value>) -> anyhow::Result<()> {
        self.settings.extend(settings);
        info!("⚙️  Updated settings for module '{}'", self.info.name);
        Ok(())
    }
    
    async fn cleanup(&mut self) -> anyhow::Result<()> {
        self.initialized = false;
        info!("🧹 Cleaned up module '{}'", self.info.name);
        Ok(())
    }
}