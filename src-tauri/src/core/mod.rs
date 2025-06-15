// /Users/rafaelreis/R5 Flowlight/src-tauri/src/core/mod.rs
//! Core functionality module
//!
//! Este módulo contém as funcionalidades principais do R5 Flowlight:
//! - Motor de busca (Tantivy)
//! - Indexação de arquivos e aplicativos
//! - Gerenciamento de hotkeys globais

pub mod search_engine;
pub mod file_indexer;
pub mod app_indexer;
pub mod hotkey_manager;

// ✅ CORREÇÃO: Re-exports ativos para facilitar o uso
pub use search_engine::{SearchEngine, SearchResult, AppResult};
pub use file_indexer::FileIndexer;
// pub use app_indexer::AppIndexer; // Descomente quando implementado
// pub use hotkey_manager::HotkeyManager; // Descomente quando implementado
