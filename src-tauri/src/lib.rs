//! R5 Flowlight - Core Library
//! 
//! Este módulo contém as funcionalidades principais do R5 Flowlight,
//! incluindo motor de busca, indexação, IA e módulos especializados.

pub mod core;
pub mod api;
pub mod modules;
pub mod ai;
pub mod utils;

// Re-exportar tipos principais para facilitar o uso
pub use crate::core::search_engine::SearchEngine;
pub use crate::utils::config::AppConfig;
pub use crate::api::search::{SearchResult, AppResult};

/// Versão da aplicação
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Nome da aplicação
pub const APP_NAME: &str = "R5 Flowlight";

/// Identificador da aplicação
pub const APP_ID: &str = "com.r5hub.flowlight";