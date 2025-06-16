// Test IPC Communication

use ipc_communication::{test_basic_ipc, IPCMessage};
use log::info;

fn main() {
    env_logger::init();
    
    println!("🚀 R5 Flowlight - IPC Communication Test");
    println!("==========================================");
    
    // Test 1: Basic IPC
    match test_basic_ipc() {
        Ok(_) => {
            println!("✅ IPC Communication: PASSED");
        }
        Err(e) => {
            println!("❌ IPC Communication: FAILED");
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
    
    // Test 2: Message serialization
    println!("\n🧪 Testing message serialization...");
    
    let messages = vec![
        IPCMessage::Ping,
        IPCMessage::Pong,
        IPCMessage::ToggleOverlay,
        IPCMessage::ShowOverlay { query: Some("test".to_string()) },
        IPCMessage::HideOverlay,
        IPCMessage::SearchQuery { 
            query: "test search".to_string(), 
            session_id: "session-123".to_string() 
        },
        IPCMessage::UpdateModule { module_id: "daily".to_string() },
        IPCMessage::GetCurrentModule,
    ];
    
    for (i, message) in messages.iter().enumerate() {
        match serde_json::to_string(message) {
            Ok(json) => {
                match serde_json::from_str::<IPCMessage>(&json) {
                    Ok(_) => println!("✅ Message {}: Serialization OK", i + 1),
                    Err(e) => {
                        println!("❌ Message {}: Deserialization failed: {}", i + 1, e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                println!("❌ Message {}: Serialization failed: {}", i + 1, e);
                std::process::exit(1);
            }
        }
    }
    
    println!("\n🎉 All IPC tests completed successfully!");
    println!("✅ Ready to proceed with search-daemon testing");
}