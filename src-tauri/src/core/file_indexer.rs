// /Users/rafaelreis/R5 Flowlight/src-tauri/src/core/file_indexer.rs - VERSÃO FINAL CORRIGIDA

use anyhow::Result;
use log::{info, warn}; // Removido `error` que não era usado, limpando um warning.
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use crate::core::search_engine::SearchEngine;
use crate::utils::config::AppConfig;
use crate::utils::config_ext::AppConfigExt;

#[derive(Clone)]
pub struct FileIndexer {
    search_engine: Arc<SearchEngine>,
    config: AppConfig,
}

impl FileIndexer {
    pub fn new(search_engine: Arc<SearchEngine>) -> Self {
        let config = AppConfig::load().unwrap_or_default();
        Self { search_engine, config }
    }

    pub async fn start_indexing(&self) -> Result<()> {
        info!("Starting file indexing...");
        let paths_to_index = self.config.search.indexed_paths.clone();
        for path_str in paths_to_index {
            self.index_folder(PathBuf::from(path_str)).await?;
        }
        self.search_engine.commit_changes().await?;
        info!("Initial indexing commit complete.");
        Ok(())
    }

    fn index_folder<'a>(&'a self, path: PathBuf) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            if !path.is_dir() { return Ok(()); }
            let mut entries = fs::read_dir(path).await?;
            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    self.index_folder(entry_path).await?;
                } else if entry_path.is_file() && self.config.should_index_file(&entry_path) {
                    if let Err(e) = self.index_file(&entry_path).await {
                        warn!("Could not index file '{}': {}", entry_path.display(), e);
                    }
                }
            }
            Ok(())
        })
    }

    async fn index_file(&self, path: &Path) -> Result<()> {
        let path_str = path.to_string_lossy().to_string();
        let title = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        self.search_engine.add_document(path_str, title).await?;
        Ok(())
    }
}
