//! Context manager for AI conversations
//! 
//! Gerenciador de contexto para conversas de IA

use crate::ai::{ConversationContext, ChatMessage, MessageRole};
use log::info;

/// Gerenciador de contexto
pub struct ContextManager {
    conversations: std::collections::HashMap<String, ConversationContext>,
}

impl ContextManager {
    /// Cria uma nova instância do gerenciador
    pub fn new() -> Self {
        info!("Context manager initialized");
        Self {
            conversations: std::collections::HashMap::new(),
        }
    }
    
    /// Adiciona uma mensagem ao contexto
    pub fn add_message(&mut self, session_id: &str, role: MessageRole, content: String) {
        let context = self.conversations.entry(session_id.to_string())
            .or_insert_with(ConversationContext::default);
        
        context.messages.push(ChatMessage {
            role,
            content,
            timestamp: chrono::Utc::now(),
        });
    }
    
    /// Obtém o contexto de uma sessão
    pub fn get_context(&self, session_id: &str) -> Option<&ConversationContext> {
        self.conversations.get(session_id)
    }
    
    /// Limpa o contexto de uma sessão
    pub fn clear_context(&mut self, session_id: &str) {
        self.conversations.remove(session_id);
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}