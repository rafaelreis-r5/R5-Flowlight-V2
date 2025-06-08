//! Health module
//! 
//! Módulo de medicina clínica e do trabalho

use crate::modules::{Module, ModuleType};

pub struct HealthModule;

impl Module for HealthModule {
    fn get_type(&self) -> ModuleType {
        ModuleType::Health
    }
    
    fn get_name(&self) -> &'static str {
        "Flowlight Health"
    }
    
    fn get_description(&self) -> &'static str {
        "Medicina clínica e do trabalho"
    }
    
    fn get_icon(&self) -> &'static str {
        "⚕️"
    }
    
    fn get_hotkeys(&self) -> Vec<&'static str> {
        vec![
            "⌘+1: Symptom Checker",
            "⌘+2: Occupational Guide", 
            "⌘+3: Drug Reference",
            "⌘+4: Protocol Templates",
            "⌘+5: Health News"
        ]
    }
}