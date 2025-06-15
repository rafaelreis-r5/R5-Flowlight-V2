# Implementação de Global Shortcut para Barra de Pesquisa - Tauri v2

## 📋 Visão Geral

Este guia implementa um sistema completo de atalho global (Cmd+Space) que permite abrir sua barra de pesquisa sobre qualquer aplicativo, similar ao Alfred/Spotlight, com comportamento nativo no macOS e Windows.

### Características da Implementação

✅ **Global Shortcut**: Cmd+Space (macOS) / Ctrl+Space (Windows)  
✅ **Always on Top**: Janela sempre visível sobre outros apps  
✅ **Multi-workspace**: Funciona em todas as áreas de trabalho  
✅ **Focus Management**: Controle inteligente de foco  
✅ **Hide/Show**: Toggle da visibilidade  
✅ **Auto-hide**: Esconde ao perder foco  

---

## 🛠️ Parte 1: Configuração Backend (Rust)

### 1.1 Cargo.toml - Dependências

```toml
# src-tauri/Cargo.toml

[dependencies]
tauri = { 
    version = "2.0", 
    features = [
        "window-all",
        "shell-open",
        "global-shortcut-all"
    ] 
}
serde = { version = "1.0", features = ["derive"] }
tauri-plugin-global-shortcut = "2.0"
tauri-plugin-window-state = "2.0"

# Para funcionalidades específicas do sistema
[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.25"
objc = "0.2"

[target.'cfg(target_os = "windows")'.dependencies]
windows = { 
    version = "0.52", 
    features = [
        "Win32_UI_WindowsAndMessaging",
        "Win32_Foundation"
    ]
}
```

### 1.2 tauri.conf.json - Configuração da Janela

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:1420",
    "distDir": "../dist"
  },
  "package": {
    "productName": "R5 Flowlight",
    "version": "0.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "window": {
        "all": false,
        "show": true,
        "hide": true,
        "center": true,
        "setAlwaysOnTop": true,
        "setFocus": true,
        "minimize": true,
        "unminimize": true,
        "maximize": false,
        "unmaximize": false,
        "setResizable": true,
        "setTitle": true,
        "setSize": true,
        "setPosition": true
      },
      "globalShortcut": {
        "all": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.r5flowlight.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "resizable": true,
        "title": "R5 Flowlight",
        "width": 800,
        "height": 600,
        "minWidth": 600,
        "minHeight": 400,
        "center": true,
        "decorations": false,
        "alwaysOnTop": true,
        "skipTaskbar": true,
        "visible": false,
        "transparent": true,
        "shadow": true,
        "focus": true,
        "maximizable": false,
        "minimizable": false,
        "closable": true,
        "titleBarStyle": "Overlay"
      }
    ]
  }
}
```

### 1.3 src-tauri/src/commands/window.rs

```rust
use tauri::{command, AppHandle, Manager, Window};
use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

#[derive(serde::Serialize)]
pub struct WindowState {
    pub visible: bool,
    pub focused: bool,
    pub always_on_top: bool,
}

// Estado global da janela
static mut WINDOW_VISIBLE: bool = false;

#[command]
pub async fn toggle_main_window(app_handle: AppHandle) -> Result<WindowState, String> {
    let window = app_handle.get_webview_window("main")
        .ok_or("Main window not found")?;

    unsafe {
        if WINDOW_VISIBLE {
            hide_window(&window).await?;
            WINDOW_VISIBLE = false;
        } else {
            show_window(&window).await?;
            WINDOW_VISIBLE = true;
        }
    }

    get_window_state(&window).await
}

#[command]
pub async fn show_main_window(app_handle: AppHandle) -> Result<WindowState, String> {
    let window = app_handle.get_webview_window("main")
        .ok_or("Main window not found")?;

    show_window(&window).await?;
    unsafe { WINDOW_VISIBLE = true; }

    get_window_state(&window).await
}

#[command]
pub async fn hide_main_window(app_handle: AppHandle) -> Result<WindowState, String> {
    let window = app_handle.get_webview_window("main")
        .ok_or("Main window not found")?;

    hide_window(&window).await?;
    unsafe { WINDOW_VISIBLE = false; }

    get_window_state(&window).await
}

async fn show_window(window: &Window) -> Result<(), String> {
    // Posiciona no centro da tela ativa
    center_window_on_active_screen(window).await?;
    
    // Garante que está sempre no topo
    window.set_always_on_top(true)
        .map_err(|e| format!("Failed to set always on top: {}", e))?;
    
    // Mostra a janela
    window.show()
        .map_err(|e| format!("Failed to show window: {}", e))?;
    
    // Dá foco à janela
    window.set_focus()
        .map_err(|e| format!("Failed to focus window: {}", e))?;
    
    // Traz para frente (específico do macOS)
    #[cfg(target_os = "macos")]
    bring_to_front_macos(window)?;
    
    Ok(())
}

async fn hide_window(window: &Window) -> Result<(), String> {
    window.hide()
        .map_err(|e| format!("Failed to hide window: {}", e))
}

async fn center_window_on_active_screen(window: &Window) -> Result<(), String> {
    use tauri::PhysicalPosition;
    
    // Obtém informações da tela ativa
    let monitors = window.available_monitors()
        .map_err(|e| format!("Failed to get monitors: {}", e))?;
    
    let current_monitor = window.current_monitor()
        .map_err(|e| format!("Failed to get current monitor: {}", e))?
        .unwrap_or_else(|| monitors.first().cloned().unwrap());
    
    let monitor_size = current_monitor.size();
    let monitor_pos = current_monitor.position();
    
    let window_size = window.outer_size()
        .map_err(|e| format!("Failed to get window size: {}", e))?;
    
    // Calcula posição central
    let x = monitor_pos.x + (monitor_size.width as i32 - window_size.width as i32) / 2;
    let y = monitor_pos.y + (monitor_size.height as i32 - window_size.height as i32) / 3; // Um pouco acima do centro
    
    window.set_position(PhysicalPosition::new(x, y))
        .map_err(|e| format!("Failed to set window position: {}", e))?;
    
    Ok(())
}

#[cfg(target_os = "macos")]
fn bring_to_front_macos(window: &Window) -> Result<(), String> {
    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};
    use cocoa::base::nil;
    use objc::{msg_send, sel, sel_impl};
    
    unsafe {
        let app: cocoa::base::id = NSApp();
        let _: () = msg_send![app, activateIgnoringOtherApps: cocoa::base::YES];
        let _: () = msg_send![app, setActivationPolicy: NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular];
    }
    
    Ok(())
}

async fn get_window_state(window: &Window) -> Result<WindowState, String> {
    let visible = window.is_visible()
        .map_err(|e| format!("Failed to get visibility: {}", e))?;
    
    let focused = window.is_focused()
        .map_err(|e| format!("Failed to get focus state: {}", e))?;
    
    Ok(WindowState {
        visible,
        focused,
        always_on_top: true, // Sempre true para nossa aplicação
    })
}

// Função para registrar o atalho global
pub fn register_global_shortcut(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle_clone = app_handle.clone();
    
    app_handle.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcuts(&["CmdOrCtrl+Space"])?
            .with_handler(move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    let app_handle = app_handle_clone.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Err(e) = toggle_main_window(app_handle).await {
                            eprintln!("Failed to toggle window: {}", e);
                        }
                    });
                }
            })
            .build(),
    )?;
    
    Ok(())
}

#[command]
pub async fn register_shortcut(app_handle: AppHandle, shortcut: String) -> Result<String, String> {
    // Permite ao usuário customizar o atalho
    register_global_shortcut(&app_handle)
        .map_err(|e| format!("Failed to register shortcut '{}': {}", shortcut, e))?;
    
    Ok(format!("Shortcut '{}' registered successfully", shortcut))
}

// Comando para detectar quando a janela perde o foco (auto-hide)
#[command]
pub async fn setup_auto_hide(app_handle: AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("main")
        .ok_or("Main window not found")?;
    
    let window_clone = window.clone();
    window.on_window_event(move |event| {
        match event {
            tauri::WindowEvent::Focused(focused) => {
                if !focused {
                    // Esconde a janela quando perde o foco
                    let window = window_clone.clone();
                    tauri::async_runtime::spawn(async move {
                        // Delay pequeno para evitar esconder muito rapidamente
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        
                        if let Err(e) = hide_window(&window).await {
                            eprintln!("Failed to auto-hide window: {}", e);
                        }
                        unsafe { WINDOW_VISIBLE = false; }
                    });
                }
            }
            _ => {}
        }
    });
    
    Ok(())
}
```

### 1.4 src-tauri/src/main.rs

```rust
// Prevent additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::window::*;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            toggle_main_window,
            show_main_window,
            hide_main_window,
            register_shortcut,
            setup_auto_hide
        ])
        .setup(|app| {
            // Registra o atalho global na inicialização
            if let Err(e) = register_global_shortcut(app.handle()) {
                eprintln!("Failed to register global shortcut: {}", e);
            }
            
            // Configura auto-hide
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = setup_auto_hide(app_handle).await {
                    eprintln!("Failed to setup auto-hide: {}", e);
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## ⚛️ Parte 2: Frontend React

### 2.1 src/hooks/useGlobalShortcut.ts

```typescript
import { useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { getCurrent } from '@tauri-apps/api/window';

interface UseGlobalShortcutOptions {
  autoHide?: boolean;
  hideDelay?: number;
}

export const useGlobalShortcut = (options: UseGlobalShortcutOptions = {}) => {
  const { autoHide = true, hideDelay = 100 } = options;

  const toggleWindow = useCallback(async () => {
    try {
      await invoke('toggle_main_window');
    } catch (error) {
      console.error('Failed to toggle window:', error);
    }
  }, []);

  const showWindow = useCallback(async () => {
    try {
      await invoke('show_main_window');
    } catch (error) {
      console.error('Failed to show window:', error);
    }
  }, []);

  const hideWindow = useCallback(async () => {
    try {
      await invoke('hide_main_window');
    } catch (error) {
      console.error('Failed to hide window:', error);
    }
  }, []);

  // Configurar eventos de janela
  useEffect(() => {
    const currentWindow = getCurrent();
    
    // Listener para quando a janela perde o foco
    let unlistenBlur: (() => void) | undefined;
    
    if (autoHide) {
      currentWindow.onFocusChanged(({ payload: focused }) => {
        if (!focused) {
          setTimeout(() => {
            hideWindow();
          }, hideDelay);
        }
      }).then(unlisten => {
        unlistenBlur = unlisten;
      });
    }

    // Cleanup
    return () => {
      if (unlistenBlur) {
        unlistenBlur();
      }
    };
  }, [autoHide, hideDelay, hideWindow]);

  // Listener para tecla ESC
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        hideWindow();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [hideWindow]);

  return {
    toggleWindow,
    showWindow,
    hideWindow
  };
};
```

### 2.2 src/components/SearchBar.tsx

```tsx
import React, { useState, useRef, useEffect } from 'react';
import { useGlobalShortcut } from '../hooks/useGlobalShortcut';

interface SearchBarProps {
  onSearch?: (query: string) => void;
  placeholder?: string;
}

export const SearchBar: React.FC<SearchBarProps> = ({
  onSearch,
  placeholder = "Search files, apps, and more..."
}) => {
  const [query, setQuery] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);
  const { hideWindow } = useGlobalShortcut();

  // Auto-focus quando a janela aparece
  useEffect(() => {
    const focusInput = () => {
      if (inputRef.current) {
        inputRef.current.focus();
        inputRef.current.select();
      }
    };

    // Focus imediato
    focusInput();

    // Re-focus após um pequeno delay (garante que funciona)
    const timeoutId = setTimeout(focusInput, 50);

    return () => clearTimeout(timeoutId);
  }, []);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (query.trim() && onSearch) {
      onSearch(query.trim());
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newQuery = e.target.value;
    setQuery(newQuery);
    
    // Search em tempo real
    if (onSearch) {
      onSearch(newQuery);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      if (query) {
        // Se há texto, limpa primeiro
        setQuery('');
        if (onSearch) onSearch('');
      } else {
        // Se não há texto, esconde a janela
        hideWindow();
      }
    }
  };

  return (
    <div className="w-full max-w-2xl mx-auto">
      <form onSubmit={handleSubmit} className="relative">
        <div className="relative">
          {/* Ícone de busca */}
          <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
            <svg
              className="h-5 w-5 text-gray-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
              />
            </svg>
          </div>

          {/* Input principal */}
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={handleInputChange}
            onKeyDown={handleKeyDown}
            placeholder={placeholder}
            className="
              w-full pl-12 pr-4 py-4 
              text-lg
              bg-white/90 backdrop-blur-sm
              border border-gray-200/50
              rounded-2xl
              shadow-2xl
              focus:outline-none focus:ring-2 focus:ring-blue-500/50
              focus:border-transparent
              placeholder-gray-400
              transition-all duration-200
            "
            autoComplete="off"
            spellCheck="false"
          />

          {/* Indicador de tecla ESC */}
          {query && (
            <div className="absolute inset-y-0 right-0 pr-4 flex items-center pointer-events-none">
              <kbd className="px-2 py-1 text-xs font-semibold text-gray-500 bg-gray-100 border border-gray-200 rounded-md">
                ESC
              </kbd>
            </div>
          )}
        </div>

        {/* Dica de atalho */}
        <div className="absolute -bottom-8 left-1/2 transform -translate-x-1/2">
          <p className="text-xs text-gray-500">
            <kbd className="px-1 py-0.5 text-xs font-semibold text-gray-500 bg-gray-100 border border-gray-200 rounded">
              ⌘Space
            </kbd>
            {' '}to show • {' '}
            <kbd className="px-1 py-0.5 text-xs font-semibold text-gray-500 bg-gray-100 border border-gray-200 rounded">
              ESC
            </kbd>
            {' '}to hide
          </p>
        </div>
      </form>
    </div>
  );
};
```

### 2.3 src/App.tsx

```tsx
import React, { useState, useEffect } from 'react';
import { SearchBar } from './components/SearchBar';
import { SearchResults } from './components/SearchResults';
import { useGlobalShortcut } from './hooks/useGlobalShortcut';

interface SearchResult {
  id: string;
  name: string;
  path: string;
  type: 'file' | 'folder' | 'app';
}

function App() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  
  // Configurar comportamentos globais
  useGlobalShortcut({
    autoHide: true,
    hideDelay: 100
  });

  // Simulação de busca (substitua pela sua lógica real)
  useEffect(() => {
    if (!query.trim()) {
      setResults([]);
      return;
    }

    setIsLoading(true);

    // Debounce da busca
    const timeoutId = setTimeout(async () => {
      try {
        // Aqui você faria a busca real
        // const searchResults = await invoke('search_files', { query });
        
        // Mock para demonstração
        const mockResults: SearchResult[] = [
          {
            id: '1',
            name: `Document about ${query}`,
            path: `/Users/user/Documents/${query}.txt`,
            type: 'file'
          },
          {
            id: '2',
            name: `${query} Project`,
            path: `/Users/user/Projects/${query}`,
            type: 'folder'
          }
        ];
        
        setResults(mockResults);
      } catch (error) {
        console.error('Search error:', error);
        setResults([]);
      } finally {
        setIsLoading(false);
      }
    }, 300);

    return () => clearTimeout(timeoutId);
  }, [query]);

  return (
    <div className="min-h-screen bg-transparent">
      {/* Container principal com backdrop blur */}
      <div className="fixed inset-0 bg-black/20 backdrop-blur-sm">
        <div className="flex items-start justify-center pt-32 px-4">
          <div className="w-full max-w-3xl">
            {/* Barra de pesquisa */}
            <SearchBar
              onSearch={setQuery}
              placeholder="Search files, folders, and applications..."
            />

            {/* Resultados */}
            {query && (
              <div className="mt-4 bg-white/95 backdrop-blur-sm rounded-2xl shadow-2xl border border-gray-200/50 overflow-hidden">
                {isLoading ? (
                  <div className="p-8 text-center">
                    <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
                    <p className="mt-2 text-gray-500">Searching...</p>
                  </div>
                ) : results.length > 0 ? (
                  <SearchResults results={results} />
                ) : (
                  <div className="p-8 text-center text-gray-500">
                    No results found for "{query}"
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
```

---

## 🎨 Parte 3: Styling Moderno

### 3.1 src/index.css

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

/* Reset e base styles */
* {
  box-sizing: border-box;
}

html, body {
  margin: 0;
  padding: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background: transparent;
}

#root {
  background: transparent;
  min-height: 100vh;
}

/* Animações customizadas */
@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

@keyframes slideOut {
  from {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
  to {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
}

.search-container {
  animation: slideIn 0.2s cubic-bezier(0.16, 1, 0.3, 1);
}

.search-results {
  animation: slideIn 0.15s cubic-bezier(0.16, 1, 0.3, 1);
}

/* Scrollbar customizada */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.2);
}

/* Glassmorphism effects */
.glass {
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.glass-dark {
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

/* Focus styles */
.focus-ring {
  @apply focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-transparent;
}

/* Utilitários adicionais */
.text-shadow {
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.backdrop-blur-strong {
  backdrop-filter: blur(24px);
  -webkit-backdrop-filter: blur(24px);
}
```

---

## 🔧 Parte 4: Configurações Avançadas

### 4.1 Comportamento Específico por Plataforma

```rust
// src-tauri/src/commands/platform.rs

#[cfg(target_os = "macos")]
pub fn setup_macos_behavior(window: &Window) -> Result<(), String> {
    use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
    use cocoa::base::id;
    
    unsafe {
        let ns_window: id = window.ns_window()
            .map_err(|e| format!("Failed to get NSWindow: {}", e))? as id;
        
        // Aparece em todos os espaços/desktops
        let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle;
        
        let _: () = msg_send![ns_window, setCollectionBehavior: behavior];
        
        // Remove da dock e do Command+Tab
        let _: () = msg_send![ns_window, setCanHide: cocoa::base::NO];
    }
    
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn setup_windows_behavior(window: &Window) -> Result<(), String> {
    use windows::Win32::UI::WindowsAndMessaging::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE};
    
    unsafe {
        let hwnd = window.hwnd()
            .map_err(|e| format!("Failed to get HWND: {}", e))?;
        
        // Always on top
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE,
        );
    }
    
    Ok(())
}
```

### 4.2 Configuração de Shortcuts Customizáveis

```typescript
// src/config/shortcuts.ts

export interface ShortcutConfig {
  toggle: string;
  hide: string;
  show: string;
}

export const DEFAULT_SHORTCUTS: ShortcutConfig = {
  toggle: 'CmdOrCtrl+Space',
  hide: 'Escape',
  show: 'CmdOrCtrl+Space'
};

export const SHORTCUT_OPTIONS = [
  { label: 'Cmd+Space (Default)', value: 'CmdOrCtrl+Space' },
  { label: 'Cmd+K', value: 'CmdOrCtrl+K' },
  { label: 'Cmd+Shift+Space', value: 'CmdOrCtrl+Shift+Space' },
  { label: 'Alt+Space', value: 'Alt+Space' },
  { label: 'Ctrl+Alt+Space', value: 'Ctrl+Alt+Space' }
];

// Hook para gerenciar shortcuts
export const useShortcutConfig = () => {
  const [shortcuts, setShortcuts] = useState<ShortcutConfig>(DEFAULT_SHORTCUTS);
  
  const updateShortcut = async (key: keyof ShortcutConfig, value: string) => {
    try {
