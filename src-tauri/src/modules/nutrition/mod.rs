//! Nutrition module
//! 
//! Módulo de planejamento nutricional e dietético

use crate::modules::{Module, ModuleType};

pub struct NutritionModule;

impl Module for NutritionModule {
    fn get_type(&self) -> ModuleType {
        ModuleType::Nutrition
    }
    
    fn get_name(&self) -> &'static str {
        "Flowlight Nutrition"
    }
    
    fn get_description(&self) -> &'static str {
        "Planejamento nutricional e dietético"
    }
    
    fn get_icon(&self) -> &'static str {
        "🥗"
    }
    
    fn get_hotkeys(&self) -> Vec<&'static str> {
        vec![
            "⌘+1: Macro Tracker",
            "⌘+2: Meal Planner", 
            "⌘+3: Recipe Generator",
            "⌘+4: Nutrient Glossary",
            "⌘+5: Grocery List"
        ]
    }
}