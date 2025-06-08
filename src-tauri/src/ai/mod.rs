//! AI integration module
//! 
//! Este módulo contém a integração com provedores de IA:
//! - OpenAI GPT-4
//! - Anthropic Claude
//! - Modelos locais (Lightning AI)
//! - Gerenciamento de contexto
//! - Templates de prompts

pub mod providers;
pub mod context_manager;
pub mod prompt_templates;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Provedores de IA disponíveis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Local,
}

/// Erros de IA
#[derive(Error, Debug)]
pub enum AIError {
    #[error("Provider not available: {0}")]
    ProviderUnavailable(String),
    
    #[error("API key missing for provider: {0}")]
    ApiKeyMissing(String),
    
    #[error("Request failed: {0}")]
    RequestFailed(String),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Context too long: {0} tokens")]
    ContextTooLong(usize),
}

/// Resposta de IA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub provider: AIProvider,
    pub tokens_used: Option<usize>,
    pub model: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Contexto de conversa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub module_type: Option<crate::modules::ModuleType>,
    pub messages: Vec<ChatMessage>,
    pub system_prompt: Option<String>,
    pub max_tokens: usize,
}

/// Mensagem de chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Papel da mensagem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

impl Default for ConversationContext {
    fn default() -> Self {
        Self {
            module_type: None,
            messages: Vec::new(),
            system_prompt: None,
            max_tokens: 4096,
        }
    }
}