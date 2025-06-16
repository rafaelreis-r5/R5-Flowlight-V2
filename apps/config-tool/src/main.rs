// R5 Flowlight Configuration Tool

use clap::{Parser, Subcommand};
use colored::*;
use shared_core::ConfigManager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show current configuration
    Show,
    /// Set global shortcut
    SetShortcut { shortcut: String },
    /// Enable/disable a module
    Module { 
        module_id: String,
        #[arg(short, long)]
        enable: Option<bool>,
    },
    /// Set UI theme
    SetTheme { theme: String },
    /// Set overlay size
    SetSize { width: f64, height: f64 },
    /// Export configuration to file
    Export { path: String },
    /// Import configuration from file
    Import { path: String },
    /// Reset to default configuration
    Reset,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    let mut config_manager = ConfigManager::new()?;
    
    match cli.command {
        Commands::Show => show_config(&config_manager),
        Commands::SetShortcut { shortcut } => {
            config_manager.set_global_shortcut(shortcut.clone())?;
            println!("{} Global shortcut updated to: {}", 
                "✅".green(), 
                shortcut.cyan().bold()
            );
        }
        Commands::Module { module_id, enable } => {
            if let Some(enabled) = enable {
                config_manager.set_module_enabled(&module_id, enabled)?;
                println!("{} Module '{}' {}", 
                    "✅".green(), 
                    module_id.cyan().bold(),
                    if enabled { "enabled".green() } else { "disabled".red() }
                );
            } else {
                show_module_config(&config_manager, &module_id);
            }
        }
        Commands::SetTheme { theme } => {
            config_manager.set_theme(theme.clone())?;
            println!("{} Theme updated to: {}", 
                "✅".green(), 
                theme.cyan().bold()
            );
        }
        Commands::SetSize { width, height } => {
            config_manager.set_overlay_size(width, height)?;
            println!("{} Overlay size updated to: {}x{}", 
                "✅".green(), 
                width.to_string().cyan().bold(),
                height.to_string().cyan().bold()
            );
        }
        Commands::Export { path } => {
            let path_buf = std::path::PathBuf::from(path.clone());
            config_manager.export_config(&path_buf)?;
            println!("{} Configuration exported to: {}", 
                "✅".green(), 
                path.cyan().bold()
            );
        }
        Commands::Import { path } => {
            let path_buf = std::path::PathBuf::from(path.clone());
            config_manager.import_config(&path_buf)?;
            println!("{} Configuration imported from: {}", 
                "✅".green(), 
                path.cyan().bold()
            );
        }
        Commands::Reset => {
            // Create new default config and save
            let default_config = shared_core::R5Config::default();
            *config_manager.get_config_mut() = default_config;
            config_manager.save()?;
            println!("{} Configuration reset to defaults", "✅".green());
        }
    }
    
    Ok(())
}

fn show_config(config_manager: &ConfigManager) {
    let config = config_manager.get_config();
    
    println!("\n{}", "🔧 R5 Flowlight Configuration".cyan().bold());
    println!("{}", "═".repeat(40).dimmed());
    
    // Shortcuts
    println!("\n{}", "⌨️  Shortcuts".yellow().bold());
    println!("  Global Shortcut: {}", config.shortcuts.global_shortcut.green());
    println!("  Show Overlay:    {}", config.shortcuts.show_overlay.green());
    println!("  Hide Overlay:    {}", config.shortcuts.hide_overlay.green());
    println!("  Next Result:     {}", config.shortcuts.next_result.green());
    println!("  Prev Result:     {}", config.shortcuts.prev_result.green());
    println!("  Execute Result:  {}", config.shortcuts.execute_result.green());
    
    // UI Settings
    println!("\n{}", "🎨 UI Settings".yellow().bold());
    println!("  Theme:           {}", config.ui.theme.green());
    println!("  Overlay Size:    {}x{}", 
        config.ui.overlay_width.to_string().green(),
        config.ui.overlay_height.to_string().green()
    );
    println!("  Max Results:     {}", config.ui.max_results.to_string().green());
    println!("  Animation Speed: {}", config.ui.animation_speed.to_string().green());
    println!("  Auto Hide Delay: {}ms", config.ui.auto_hide_delay.to_string().green());
    
    // Modules
    println!("\n{}", "📦 Modules".yellow().bold());
    for (module_id, module_config) in &config.modules {
        let status = if module_config.enabled { 
            "enabled".green() 
        } else { 
            "disabled".red() 
        };
        println!("  {}: {} (priority: {})", 
            module_id.cyan().bold(),
            status,
            module_config.priority.to_string().dimmed()
        );
        
        if !module_config.settings.is_empty() {
            for (key, value) in &module_config.settings {
                println!("    {}: {}", 
                    key.dimmed(),
                    value.to_string().trim_matches('"').dimmed()
                );
            }
        }
    }
    
    // App Config
    if let Some(current_module) = &config.app.current_module {
        println!("\n{}", "🎯 Current Module".yellow().bold());
        println!("  Active: {}", current_module.cyan().bold());
    }
    
    // Config Path
    println!("\n{}", "📁 Configuration".yellow().bold());
    println!("  Path: {}", config_manager.get_config_file_path().to_string_lossy().dimmed());
    
    println!("\n{}", "═".repeat(40).dimmed());
    println!("{}", "Use 'config-tool --help' for available commands".dimmed());
}

fn show_module_config(config_manager: &ConfigManager, module_id: &str) {
    if let Some(module_config) = config_manager.get_module_config(module_id) {
        println!("\n{} {}", "📦 Module:".yellow().bold(), module_id.cyan().bold());
        println!("{}", "─".repeat(30).dimmed());
        
        let status = if module_config.enabled { 
            "enabled".green() 
        } else { 
            "disabled".red() 
        };
        println!("Status:   {}", status);
        println!("Priority: {}", module_config.priority.to_string().cyan());
        
        if !module_config.settings.is_empty() {
            println!("\n{}", "Settings:".yellow());
            for (key, value) in &module_config.settings {
                println!("  {}: {}", 
                    key.cyan(),
                    value.to_string().trim_matches('"').green()
                );
            }
        }
    } else {
        println!("{} Module '{}' not found", "❌".red(), module_id.red());
    }
}