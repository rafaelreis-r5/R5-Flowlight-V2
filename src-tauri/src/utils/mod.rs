//! Utilities module
//! 
//! Este módulo contém utilitários compartilhados:
//! - Configuração da aplicação
//! - Logger
//! - Criptografia
//! - Helpers diversos
//! - Cliente Supabase

pub mod config;
pub mod config_ext;
pub mod logger;
pub mod crypto;
pub mod supabase_client;

// Re-exports
// pub use config::AppConfig;