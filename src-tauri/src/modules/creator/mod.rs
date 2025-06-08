//! Creator module
//! 
//! Módulo de criação de conteúdo e marketing

use crate::modules::{Module, ModuleType};

pub struct CreatorModule;

impl Module for CreatorModule {
    fn get_type(&self) -> ModuleType {
        ModuleType::Creator
    }
    
    fn get_name(&self) -> &'static str {
        "Flowlight Creator"
    }
    
    fn get_description(&self) -> &'static str {
        "Criação de conteúdo e marketing"
    }
    
    fn get_icon(&self) -> &'static str {
        "🎨"
    }
    
    fn get_hotkeys(&self) -> Vec<&'static str> {
        vec![
            "⌘+1: Idea Spark",
            "⌘+2: Copy Assistant", 
            "⌘+3: Format Converter",
            "⌘+4: Metrics Dashboard",
            "⌘+5: Media Planner"
        ]
    }
}