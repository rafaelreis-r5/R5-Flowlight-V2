// Module registry for managing and executing searches across modules

use crate::traits::*;
use crate::daily::DailyModule;
use log::{info, error, warn, debug};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ModuleRegistry {
    modules: HashMap<String, Arc<RwLock<Box<dyn SearchModule>>>>,
    enabled_modules: Vec<String>,
    default_module: Option<String>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            enabled_modules: Vec::new(),
            default_module: None,
        }
    }
    
    pub async fn initialize_default_modules(&mut self) -> anyhow::Result<()> {
        info!("🏗️  Initializing default modules...");
        
        // Register daily module
        let mut daily_module = DailyModule::new();
        let config = HashMap::new(); // Use default configuration
        daily_module.initialize(config).await?;
        
        let module_id = daily_module.info().id.clone();
        self.register_module(module_id.clone(), Box::new(daily_module)).await?;
        self.set_default_module(module_id).await?;
        
        info!("✅ Default modules initialized successfully");
        Ok(())
    }
    
    pub async fn register_module(
        &mut self,
        module_id: String,
        module: Box<dyn SearchModule>,
    ) -> anyhow::Result<()> {
        let module_info = module.info();
        
        if module_info.enabled {
            self.enabled_modules.push(module_id.clone());
        }
        
        self.modules.insert(module_id.clone(), Arc::new(RwLock::new(module)));
        
        info!("📦 Registered module: {} ({})", module_info.name, module_id);
        Ok(())
    }
    
    pub async fn unregister_module(&mut self, module_id: &str) -> anyhow::Result<()> {
        if let Some(module_arc) = self.modules.remove(module_id) {
            let mut module = module_arc.write().await;
            module.cleanup().await?;
            
            // Remove from enabled modules
            self.enabled_modules.retain(|id| id != module_id);
            
            // Clear default if this was the default module
            if self.default_module.as_ref() == Some(&module_id.to_string()) {
                self.default_module = None;
            }
            
            info!("🗑️  Unregistered module: {}", module_id);
        } else {
            warn!("⚠️  Attempted to unregister non-existent module: {}", module_id);
        }
        
        Ok(())
    }
    
    pub async fn set_default_module(&mut self, module_id: String) -> anyhow::Result<()> {
        if self.modules.contains_key(&module_id) {
            self.default_module = Some(module_id.clone());
            info!("🎯 Set default module: {}", module_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Module '{}' not found", module_id))
        }
    }
    
    pub fn get_default_module(&self) -> Option<String> {
        self.default_module.clone()
    }
    
    pub fn list_modules(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }
    
    pub fn list_enabled_modules(&self) -> Vec<String> {
        self.enabled_modules.clone()
    }
    
    pub async fn get_module_info(&self, module_id: &str) -> Option<ModuleInfo> {
        if let Some(module_arc) = self.modules.get(module_id) {
            let module = module_arc.read().await;
            Some(module.info())
        } else {
            None
        }
    }
    
    pub async fn search_all_modules(&self, query: &SearchQuery) -> anyhow::Result<Vec<SearchResult>> {
        let mut all_results = Vec::new();
        
        // If a specific module is requested, search only that module
        if let Some(module_filter) = &query.module_filter {
            if let Some(module_arc) = self.modules.get(module_filter) {
                let module = module_arc.read().await;
                let results = module.search(query).await?;
                all_results.extend(results);
                debug!("🔍 Search in module '{}' returned {} results", module_filter, all_results.len());
            } else {
                warn!("⚠️  Requested module '{}' not found", module_filter);
            }
        } else {
            // Search enabled modules
            for module_id in &self.enabled_modules {
                if let Some(module_arc) = self.modules.get(module_id) {
                    let module = module_arc.read().await;
                    match module.search(query).await {
                        Ok(results) => {
                            debug!("🔍 Module '{}' returned {} results", module_id, results.len());
                            all_results.extend(results);
                        }
                        Err(e) => {
                            error!("❌ Search failed in module '{}': {}", module_id, e);
                        }
                    }
                }
            }
        }
        
        // Sort results by score (descending)
        all_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        // Limit results
        all_results.truncate(query.max_results);
        
        info!("🔍 Total search results: {}", all_results.len());
        Ok(all_results)
    }
    
    pub async fn execute_action(&self, result_id: &str, action_type: &str) -> anyhow::Result<()> {
        // Try to find which module contains this result and execute the action
        for (module_id, module_arc) in &self.modules {
            let module = module_arc.read().await;
            
            // We need a way to determine if a result belongs to a module
            // For now, we'll try executing on all modules until one succeeds
            match module.execute_action(result_id, action_type).await {
                Ok(_) => {
                    info!("✅ Action executed successfully by module '{}'", module_id);
                    return Ok(());
                }
                Err(_) => {
                    // Continue to next module
                    debug!("Module '{}' cannot handle result '{}'", module_id, result_id);
                }
            }
        }
        
        error!("❌ No module could execute action '{}' for result '{}'", action_type, result_id);
        Err(anyhow::anyhow!("No module found to handle the action"))
    }
    
    pub async fn health_check_all(&self) -> HashMap<String, bool> {
        let mut health_status = HashMap::new();
        
        for (module_id, module_arc) in &self.modules {
            let module = module_arc.read().await;
            match module.health_check().await {
                Ok(healthy) => {
                    health_status.insert(module_id.clone(), healthy);
                }
                Err(e) => {
                    error!("❌ Health check failed for module '{}': {}", module_id, e);
                    health_status.insert(module_id.clone(), false);
                }
            }
        }
        
        info!("🔍 Health check completed for {} modules", health_status.len());
        health_status
    }
    
    pub async fn enable_module(&mut self, module_id: &str) -> anyhow::Result<()> {
        if self.modules.contains_key(module_id) {
            if !self.enabled_modules.contains(&module_id.to_string()) {
                self.enabled_modules.push(module_id.to_string());
                info!("✅ Enabled module: {}", module_id);
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Module '{}' not found", module_id))
        }
    }
    
    pub async fn disable_module(&mut self, module_id: &str) -> anyhow::Result<()> {
        self.enabled_modules.retain(|id| id != module_id);
        info!("🚫 Disabled module: {}", module_id);
        Ok(())
    }
    
    pub async fn cleanup_all(&mut self) -> anyhow::Result<()> {
        info!("🧹 Cleaning up all modules...");
        
        for (module_id, module_arc) in &self.modules {
            let mut module = module_arc.write().await;
            if let Err(e) = module.cleanup().await {
                error!("❌ Failed to cleanup module '{}': {}", module_id, e);
            }
        }
        
        self.modules.clear();
        self.enabled_modules.clear();
        self.default_module = None;
        
        info!("✅ All modules cleaned up");
        Ok(())
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}