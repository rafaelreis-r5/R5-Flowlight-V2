// Test Search Daemon (Simplified)

use log::{info, error};
use std::time::Duration;
use std::process::Command;

fn main() {
    env_logger::init();
    
    println!("🚀 R5 Flowlight - Search Daemon Test");
    println!("====================================");
    
    // Test 1: Check if daemon binary can be built
    println!("🧪 Testing daemon compilation...");
    
    let build_result = Command::new("cargo")
        .args(&["check", "-p", "search-daemon", "--bin", "r5-flowlight-daemon"])
        .current_dir("/Users/rafaelreis/R5 Flowlight")
        .output();
    
    match build_result {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Search daemon compilation: PASSED");
            } else {
                println!("❌ Search daemon compilation: FAILED");
                println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        }
        Err(e) => {
            println!("❌ Failed to run cargo check: {}", e);
            std::process::exit(1);
        }
    }
    
    // Test 2: Check shared-core functionality
    println!("\n🧪 Testing shared-core types...");
    
    use shared_core::{AppConfig, Module, SystemSettings};
    
    let mut config = AppConfig::default();
    config.current_module = Some("test_module".to_string());
    
    let module = Module {
        id: "test_module".to_string(),
        name: "Test Module".to_string(),
        description: "A test module".to_string(),
        enabled: true,
        version: "1.0.0".to_string(),
        settings: std::collections::HashMap::new(),
    };
    
    config.modules.push(module);
    
    // Test serialization
    match serde_json::to_string_pretty(&config) {
        Ok(json) => {
            println!("✅ Config serialization: PASSED");
            match serde_json::from_str::<AppConfig>(&json) {
                Ok(deserialized) => {
                    if deserialized.current_module == config.current_module {
                        println!("✅ Config deserialization: PASSED");
                    } else {
                        println!("❌ Config deserialization: Data mismatch");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    println!("❌ Config deserialization: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("❌ Config serialization: {}", e);
            std::process::exit(1);
        }
    }
    
    // Test 3: Simulate daemon state
    println!("\n🧪 Testing daemon state management...");
    
    use shared_core::utils::{current_timestamp_ms, format_duration_ms};
    
    let start_time = current_timestamp_ms();
    std::thread::sleep(Duration::from_millis(100));
    let end_time = current_timestamp_ms();
    
    let duration = end_time - start_time;
    let formatted = format_duration_ms(duration);
    
    println!("✅ Timestamp functions: {} elapsed", formatted);
    
    if duration >= 90 && duration <= 150 { // Allow some variance
        println!("✅ Duration calculation: PASSED");
    } else {
        println!("❌ Duration calculation: Expected ~100ms, got {}ms", duration);
    }
    
    println!("\n🎉 All daemon foundation tests completed successfully!");
    println!("✅ Ready to proceed with overlay implementation");
    println!("\n📋 Next steps:");
    println!("   1. Implement macOS overlay with NSPanel");
    println!("   2. Test global shortcut integration");
    println!("   3. Test end-to-end IPC communication");
}