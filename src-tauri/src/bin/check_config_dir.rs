// Em src-tauri/src/bin/check_config_dir.rs
use std::fs;
use directories::ProjectDirs;

fn main() {
    if let Some(proj_dirs) = ProjectDirs::from("com", "r5hub", "flowlight") {
        let config_dir = proj_dirs.config_dir();
        println!("Config directory: {:?}", config_dir);
        // ... (resto do seu código de teste)
    } else {
        println!("Could not determine project directories.");
    }
}
