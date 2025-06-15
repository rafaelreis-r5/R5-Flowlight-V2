//! Application configuration
//!
//! Gerenciamento da configuração da aplicação

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

/// Configuração principal da aplicação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Configurações gerais
    pub general: GeneralConfig,

    /// Configurações de busca
    pub search: SearchConfig,

    /// Configurações de IA
    pub ai: AIConfig,

    /// Configurações dos módulos
    pub modules: ModulesConfig,
}

/// Configurações gerais
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Tema da interface
    pub theme: String,

    /// Idioma da interface
    pub language: String,

    /// Iniciar com o sistema
    pub start_with_system: bool,

    /// Mostrar na bandeja do sistema
    pub show_in_tray: bool,
}

/// Configurações de busca
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Diretórios para indexar
    pub indexed_paths: Vec<PathBuf>,

    /// Diretórios para ignorar
    pub ignored_paths: Vec<PathBuf>,

    /// Extensões de arquivo para indexar
    pub indexed_extensions: Vec<String>,

    /// Tamanho máximo de arquivo (MB)
    pub max_file_size_mb: u64,

    /// Intervalo de atualização do índice (segundos)
    pub index_update_interval: u64,

    /// Incluir arquivos ocultos na indexação
    pub include_hidden: bool,
}

/// Configurações de IA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Provedor de IA padrão
    pub default_provider: String,

    /// Chave da API OpenAI
    pub openai_api_key: Option<String>,

    /// Chave da API Anthropic
    pub anthropic_api_key: Option<String>,

    /// Modelo padrão
    pub default_model: String,

    /// Temperatura padrão
    pub default_temperature: f32,

    /// Máximo de tokens
    pub max_tokens: usize,
}

/// Configurações dos módulos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulesConfig {
    /// Módulos habilitados
    pub enabled_modules: Vec<String>,

    /// Configurações específicas dos módulos
    pub module_settings: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                theme: "dark".to_string(),
                language: "pt-BR".to_string(),
                start_with_system: false,
                show_in_tray: true,
            },
            search: SearchConfig {
                indexed_paths: vec![
                    dirs::home_dir().unwrap_or_default(),
                    dirs::desktop_dir().unwrap_or_default(),
                    dirs::document_dir().unwrap_or_default(),
                ],
                ignored_paths: vec![
                    PathBuf::from(".git"),
                    PathBuf::from("node_modules"),
                    PathBuf::from(".DS_Store"),
                ],
                indexed_extensions: vec![
                    "txt".to_string(), "md".to_string(), "pdf".to_string(),
                    "doc".to_string(), "docx".to_string(), "xls".to_string(),
                    "xlsx".to_string(), "ppt".to_string(), "pptx".to_string(),
                    "js".to_string(), "ts".to_string(), "py".to_string(),
                    "rs".to_string(), "go".to_string(), "java".to_string(),
                    "c".to_string(), "cpp".to_string(), "h".to_string(),
                    "html".to_string(), "css".to_string(), "json".to_string(),
                ],
                max_file_size_mb: 50,
                index_update_interval: 300, // 5 minutos
                include_hidden: false, // Não incluir arquivos ocultos por padrão
            },
            ai: AIConfig {
                default_provider: "openai".to_string(),
                openai_api_key: None,
                anthropic_api_key: None,
                default_model: "gpt-3.5-turbo".to_string(),
                default_temperature: 0.7,
                max_tokens: 4096,
            },
            modules: ModulesConfig {
                enabled_modules: vec![
                    "finance".to_string(),
                    "health".to_string(),
                    "nutrition".to_string(),
                    "creator".to_string(),
                    "daily".to_string(),
                ],
                module_settings: std::collections::HashMap::new(),
            },
        }
    }
}

impl AppConfig {
    /// Carrega a configuração do arquivo
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();

        let mut config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            toml::from_str::<AppConfig>(&content)?
        } else {
            let default_config = Self::default();
            default_config.save()?;
            default_config
        };

        // Override AI API keys from environment variables if present
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            config.ai.openai_api_key = Some(key);
        }
        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            config.ai.anthropic_api_key = Some(key);
        }

        Ok(config)
    }

    /// Salva a configuração no arquivo
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;

        Ok(())
    }

    /// Retorna o caminho do arquivo de configuração
    fn config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("r5-flowlight").join("config.toml")
        } else {
            PathBuf::from("config.toml")
        }
    }
}
