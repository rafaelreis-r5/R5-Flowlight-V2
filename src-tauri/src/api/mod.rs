//! API endpoints module
//! 
//! Este módulo contém os handlers para as APIs do Tauri:
//! - Endpoints de busca
//! - Endpoints dos módulos
//! - Endpoints de IA

pub mod search;
pub mod modules;
pub mod ai;

// Re-exports
// pub use search::{SearchResult, AppResult};
// pub use modules::ModuleData;
// pub use ai::AIResponse;