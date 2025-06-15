//! AI providers module
//!
//! Provedores de IA: OpenAI, Anthropic, Local models

use anyhow::{Result, anyhow};
use log::info;
use crate::utils::config::AppConfig;
use reqwest::Client;
use serde_json::json;

/// Cliente OpenAI
pub struct OpenAIClient {
    pub api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        info!("OpenAI client initialized");
        Self { api_key }
    }

    pub async fn query(&self, prompt: &str) -> Result<String> {
        // Real OpenAI API call
        let config = AppConfig::load()?;
        let client = Client::new();
        let res = client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&json!({
                "model": config.ai.default_model,
                "messages": [{"role": "user", "content": prompt}],
                "temperature": config.ai.default_temperature,
                "max_tokens": config.ai.max_tokens,
            }))
            .send()
            .await?
            .error_for_status()?;
        let json: serde_json::Value = res.json().await?;
        info!("API: raw AI JSON response: {}", json);
        if let Some(err) = json.get("error") {
            let err_msg = err.get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error")
                .to_string();
            return Err(anyhow!(err_msg));
        }
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        Ok(content)
    }
}


/// Cliente de modelo local
pub struct LocalModel {
    pub model_path: String,
}

impl LocalModel {
    pub fn new(model_path: String) -> Self {
        info!("Local model initialized");
        Self { model_path }
    }

    pub async fn query(&self, _prompt: &str) -> Result<String> {
        // TODO: Implementar inferência local
        Ok("Local model response placeholder".to_string())
    }
}
