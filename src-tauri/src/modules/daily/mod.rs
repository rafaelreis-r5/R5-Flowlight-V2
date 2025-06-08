//! Daily module
//! 
//! Módulo de utilitários gerais do dia a dia

use crate::modules::{Module, ModuleType};

pub struct DailyModule;

impl Module for DailyModule {
    fn get_type(&self) -> ModuleType {
        ModuleType::Daily
    }
    
    fn get_name(&self) -> &'static str {
        "Flowlight Daily"
    }
    
    fn get_description(&self) -> &'static str {
        "Utilitários gerais do dia a dia"
    }
    
    fn get_icon(&self) -> &'static str {
        "📅"
    }
    
    fn get_hotkeys(&self) -> Vec<&'static str> {
        vec![
            "⌘+1: Quick Calc",
            "⌘+2: Unit Converter", 
            "⌘+3: Calendar Events",
            "⌘+4: Reminders & Alarms",
            "⌘+5: Clipboard History"
        ]
    }
}