// Test macOS Native Overlay

#[cfg(target_os = "macos")]
use platform_macos::test_overlay;

fn main() {
    env_logger::init();
    
    println!("🚀 R5 Flowlight - macOS Overlay Test");
    println!("====================================");
    
    #[cfg(target_os = "macos")]
    {
        match test_overlay() {
            Ok(_) => {
                println!("✅ macOS overlay: PASSED");
                println!("🎉 Overlay implementation ready for integration!");
            }
            Err(e) => {
                println!("❌ macOS overlay: FAILED");
                println!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        println!("⚠️  macOS overlay test skipped (not running on macOS)");
    }
}