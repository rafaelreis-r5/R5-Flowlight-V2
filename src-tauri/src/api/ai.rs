// em /Users/rafaelreis/R5 Flowlight/src-tauri/src/api/ai.rs
//! AI API handlers
//! Handlers para os endpoints de IA

use serde::{Deserialize, Serialize};
use log::info;
use chrono::{DateTime, Utc};
use crate::utils::config::AppConfig;
use crate::ai::providers::OpenAIClient;

/// Resposta de IA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub provider: String,
    pub model: String,
    pub tokens_used: Option<usize>,
    pub timestamp: DateTime<Utc>,
}

/// Handler para consulta de IA
pub async fn ai_query_handler(prompt: String, context: String) -> Result<String, String> {
    info!("API: AI query - prompt: {}, context: {}", prompt, context);

    if prompt.trim().is_empty() {
        return Err("Prompt vazio".to_string());
    }

    // Carrega configuração (removido warning de variável não usada)
    let _config = AppConfig::load().map_err(|e| format!("Failed to load config: {}", e))?;

        // Executa consulta via OpenAI
        let combined_prompt = format!("Context: {}\nPrompt: {}", context, prompt);
        let key = std::env::var("OPENAI_API_KEY").map_err(|_| "OpenAI API key not set".to_string())?;
        let client = OpenAIClient::new(key);
        let response = client.query(&combined_prompt).await.map_err(|e| e.to_string())?;

    info!("API: AI response: {}", response);
    Ok(response)
}
