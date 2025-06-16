// macOS Native Overlay Implementation
// Uses NSPanel for global overlay similar to Spotlight

use std::ffi::c_void;
use objc::{msg_send, sel, sel_impl, class, runtime::Object};
use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString, NSAutoreleasePool};
use cocoa::appkit::{NSPanel, NSWindow, NSView, NSScreen, NSApplication, NSApp, NSWindowStyleMask, NSBackingStoreType};

pub struct MacOSOverlay {
    panel: id,
    content_view: id,
}

impl MacOSOverlay {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            
            // Get main screen dimensions
            let main_screen = NSScreen::mainScreen(nil);
            let screen_frame = NSScreen::frame(main_screen);
            
            // Create overlay panel (400x60 pixels, centered horizontally)
            let panel_width = 400.0;
            let panel_height = 60.0;
            let panel_x = (screen_frame.size.width - panel_width) / 2.0;
            let panel_y = screen_frame.size.height - 200.0; // Top area like Spotlight
            
            let panel_rect = NSRect::new(
                NSPoint::new(panel_x, panel_y),
                NSSize::new(panel_width, panel_height)
            );
            
            // Create NSPanel with specific style
            let panel = NSPanel::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                panel_rect,
                NSWindowStyleMask::NSBorderlessWindowMask,
                NSBackingStoreType::NSBackingStoreBuffered,
                NO
            );
            
            if panel == nil {
                return Err("Failed to create NSPanel".to_string());
            }
            
            // Configure panel properties
            let _: () = msg_send![panel, setLevel: 25]; // kCGModalPanelWindowLevel
            let _: () = msg_send![panel, setOpaque: NO];
            let _: () = msg_send![panel, setHasShadow: YES];
            let _: () = msg_send![panel, setBackgroundColor: cocoa::appkit::NSColor::clearColor(nil)];
            let _: () = msg_send![panel, setFloatingPanel: YES];
            let _: () = msg_send![panel, setHidesOnDeactivate: NO];
            let _: () = msg_send![panel, setCanBecomeKeyWindow: YES];
            
            // Create content view
            let content_view = NSView::alloc(nil).initWithFrame_(panel_rect);
            if content_view == nil {
                return Err("Failed to create content view".to_string());
            }
            
            let _: () = msg_send![panel, setContentView: content_view];
            
            Ok(MacOSOverlay {
                panel,
                content_view,
            })
        }
    }
    
    pub fn show(&self) -> Result<(), String> {
        unsafe {
            let _: () = msg_send![self.panel, makeKeyAndOrderFront: nil];
            Ok(())
        }
    }
    
    pub fn hide(&self) -> Result<(), String> {
        unsafe {
            let _: () = msg_send![self.panel, orderOut: nil];
            Ok(())
        }
    }
    
    pub fn is_visible(&self) -> bool {
        unsafe {
            let visible: bool = msg_send![self.panel, isVisible];
            visible
        }
    }
    
    pub fn toggle(&self) -> Result<(), String> {
        if self.is_visible() {
            self.hide()
        } else {
            self.show()
        }
    }
    
    pub fn update_position(&self) -> Result<(), String> {
        unsafe {
            let main_screen = NSScreen::mainScreen(nil);
            let screen_frame = NSScreen::frame(main_screen);
            
            let panel_width = 400.0;
            let panel_height = 60.0;
            let panel_x = (screen_frame.size.width - panel_width) / 2.0;
            let panel_y = screen_frame.size.height - 200.0;
            
            let new_rect = NSRect::new(
                NSPoint::new(panel_x, panel_y),
                NSSize::new(panel_width, panel_height)
            );
            
            let _: () = msg_send![self.panel, setFrame: new_rect display: YES];
            Ok(())
        }
    }
}

impl Drop for MacOSOverlay {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.panel, close];
        }
    }
}

// Test function for overlay
pub fn test_overlay() -> Result<(), String> {
    println!("🍎 Testing macOS NSPanel overlay...");
    
    let overlay = MacOSOverlay::new()?;
    println!("✅ NSPanel created successfully");
    
    overlay.show()?;
    println!("✅ Overlay shown");
    
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    overlay.hide()?;
    println!("✅ Overlay hidden");
    
    println!("🎉 macOS overlay test completed!");
    Ok(())
}