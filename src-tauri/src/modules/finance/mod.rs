//! Finance module
//! 
//! Módulo de análise financeira e gestão de investimentos

use crate::modules::{Module, ModuleType};

pub struct FinanceModule;

impl Module for FinanceModule {
    fn get_type(&self) -> ModuleType {
        ModuleType::Finance
    }
    
    fn get_name(&self) -> &'static str {
        "Flowlight Finance"
    }
    
    fn get_description(&self) -> &'static str {
        "Análise financeira e gestão de investimentos"
    }
    
    fn get_icon(&self) -> &'static str {
        "💰"
    }
    
    fn get_hotkeys(&self) -> Vec<&'static str> {
        vec![
            "⌘+1: Market Overview",
            "⌘+2: Portfolio Manager", 
            "⌘+3: Trade Signal",
            "⌘+4: Financial Calculator",
            "⌘+5: News Digest"
        ]
    }
}