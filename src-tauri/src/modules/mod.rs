//! Business logic modules
//! 
//! Este módulo contém a lógica de negócio dos módulos especializados:
//! - Finance: Análise financeira e gestão de investimentos
//! - Health: Medicina clínica e do trabalho
//! - Nutrition: Planejamento nutricional e dietético
//! - Creator: Criação de conteúdo e marketing
//! - Daily: Utilitários gerais do dia a dia

pub mod finance;
pub mod health;
pub mod nutrition;
pub mod creator;
pub mod daily;

use serde::{Deserialize, Serialize};

/// Tipos de módulos disponíveis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleType {
    Finance,
    Health,
    Nutrition,
    Creator,
    Daily,
}

impl ModuleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleType::Finance => "finance",
            ModuleType::Health => "health",
            ModuleType::Nutrition => "nutrition",
            ModuleType::Creator => "creator",
            ModuleType::Daily => "daily",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "finance" => Some(ModuleType::Finance),
            "health" => Some(ModuleType::Health),
            "nutrition" => Some(ModuleType::Nutrition),
            "creator" => Some(ModuleType::Creator),
            "daily" => Some(ModuleType::Daily),
            _ => None,
        }
    }
}

/// Trait base para todos os módulos
pub trait Module {
    fn get_type(&self) -> ModuleType;
    fn get_name(&self) -> &'static str;
    fn get_description(&self) -> &'static str;
    fn get_icon(&self) -> &'static str;
    fn get_hotkeys(&self) -> Vec<&'static str>;
}