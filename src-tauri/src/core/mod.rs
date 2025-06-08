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

// Re-exports para facilitar o uso
// pub use search_engine::SearchEngine;
// pub use file_indexer::FileIndexer;
// pub use app_indexer::AppIndexer;
// pub use hotkey_manager::HotkeyManager;