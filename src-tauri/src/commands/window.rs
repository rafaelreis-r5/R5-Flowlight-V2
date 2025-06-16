use tauri::{command, AppHandle, Manager, WebviewWindow, Runtime, PhysicalPosition, Position};
use serde::{Deserialize, Serialize};
use log::{info, error, debug, warn};

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowState {
    pub visible: bool,
    pub focused: bool,
    pub always_on_top: bool,
}

// Estado global da janela de busca
static mut SEARCH_WINDOW_VISIBLE: bool = false;

#[command]
pub async fn toggle_search_window<R: Runtime>(app_handle: AppHandle<R>) -> Result<WindowState, String> {
    info!("Alternando visibilidade da janela de busca");

    // Obtém a janela de busca
    let window = match get_search_window(&app_handle) {
        Ok(w) => w,
        Err(e) => {
            error!("Erro ao obter janela de busca: {}", e);
            return Err(format!("Falha ao acessar a janela de busca: {}", e));
        }
    };

    // Verifica a visibilidade atual
    let is_visible = match window.is_visible() {
        Ok(visible) => visible,
        Err(e) => {
            error!("Erro ao verificar visibilidade da janela: {}", e);
            return Err(format!("Falha ao verificar visibilidade da janela: {}", e));
        }
    };

    info!("Visibilidade atual: {}", is_visible);

    // Alterna a visibilidade
    if is_visible {
        if let Err(e) = hide_search_window_internal(&window).await {
            error!("Erro ao esconder janela: {}", e);
            return Err(format!("Falha ao esconder a janela: {}", e));
        }
        unsafe { SEARCH_WINDOW_VISIBLE = false; }
    } else {
        if let Err(e) = show_search_window_internal(&window).await {
            error!("Erro ao mostrar janela: {}", e);
            return Err(format!("Falha ao mostrar a janela: {}", e));
        }
        unsafe { SEARCH_WINDOW_VISIBLE = true; }
    }

    // Retorna o estado atualizado da janela
    match get_window_state(&window).await {
        Ok(state) => {
            info!("Estado da janela atualizado: {:?}", state);
            Ok(state)
        }
        Err(e) => {
            error!("Erro ao obter estado da janela: {}", e);
            Err(format!("Falha ao obter estado da janela: {}", e))
        }
    }
}

#[command]
pub async fn show_search_window<R: Runtime>(app_handle: AppHandle<R>) -> Result<WindowState, String> {
    let window = get_search_window(&app_handle)?;

    show_search_window_internal(&window).await?;
    unsafe { SEARCH_WINDOW_VISIBLE = true; }

    get_window_state(&window).await
}

#[command]
pub async fn hide_search_window<R: Runtime>(app_handle: AppHandle<R>) -> Result<WindowState, String> {
    let window = get_search_window(&app_handle)?;

    hide_search_window_internal(&window).await?;
    unsafe { SEARCH_WINDOW_VISIBLE = false; }

    get_window_state(&window).await
}

#[command]
pub async fn is_search_window_visible<R: Runtime>(app_handle: AppHandle<R>) -> Result<bool, String> {
    let window = get_search_window(&app_handle)?;
    window.is_visible()
        .map_err(|e| format!("Failed to get window visibility: {}", e))
}

fn get_search_window<R: Runtime>(app_handle: &AppHandle<R>) -> Result<WebviewWindow<R>, String> {
    app_handle.get_webview_window("search")
        .ok_or("Search window not found".to_string())
}

async fn show_search_window_internal<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    // Posiciona no centro da tela ativa
    center_window_on_active_screen(window).await?;

    // Garante que está sempre no topo - CRÍTICO para sobrepor qualquer aplicação
    window.set_always_on_top(true)
        .map_err(|e| format!("Failed to set always on top: {}", e))?;

    // Mostra a janela
    window.show()
        .map_err(|e| format!("Failed to show window: {}", e))?;

    // Dá foco à janela - ESSENCIAL para receber input imediatamente
    window.set_focus()
        .map_err(|e| format!("Failed to focus window: {}", e))?;

    // ✅ CORREÇÃO: Remover operações NSWindow - elas causam crash no Tauri v2
    // configure_search_window_for_global_access(window)?;
    
    info!("Janela de busca configurada e exibida com sucesso");

    Ok(())
}

async fn hide_search_window_internal<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    window.hide()
        .map_err(|e| format!("Failed to hide window: {}", e))
}

async fn center_window_on_active_screen<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    info!("Posicionando janela no centro da tela onde usuário está trabalhando");

    // ✅ CORREÇÃO: Usar posição global do cursor, não relativa à janela
    let cursor_pos = match get_global_cursor_position() {
        Ok(pos) => {
            info!("Posição global do cursor: x={}, y={}", pos.x, pos.y);
            pos
        }
        Err(e) => {
            warn!("Falha ao obter posição global do cursor: {}. Usando fallback.", e);
            // Fallback: usar centro da tela principal
            return window.center().map_err(|e| format!("Falha ao centralizar a janela: {}", e));
        }
    };

    // Obtém os monitores disponíveis
    let monitors = window
        .available_monitors()
        .map_err(|e| format!("Falha ao obter monitores: {}", e))?;

    // Encontra o monitor ativo com base na posição GLOBAL do cursor
    for monitor in monitors {
        let pos = monitor.position();
        let size = monitor.size();

        info!("Verificando monitor: pos=({}, {}), size={}x{}", pos.x, pos.y, size.width, size.height);

        if cursor_pos.x >= pos.x as f64
            && cursor_pos.x <= (pos.x + size.width as i32) as f64
            && cursor_pos.y >= pos.y as f64
            && cursor_pos.y <= (pos.y + size.height as i32) as f64
        {
            info!("Monitor ativo encontrado! Posicionando janela...");
            
            // Obtém o tamanho da janela
            let window_size = window
                .outer_size()
                .map_err(|e| format!("Falha ao obter tamanho da janela: {}", e))?;

            // Calcula a posição central (1/3 do topo para melhor ergonomia)
            let center_x = pos.x + (size.width as i32 - window_size.width as i32) / 2;
            let center_y = pos.y + (size.height as i32 - window_size.height as i32) / 3;

            info!("Posicionando janela em: x={}, y={}", center_x, center_y);

            window
                .set_position(Position::Physical(PhysicalPosition::new(center_x, center_y)))
                .map_err(|e| format!("Falha ao posicionar a janela: {}", e))?;

            return Ok(());
        }
    }

    // Fallback: centraliza na tela principal se nenhum monitor ativo for encontrado
    warn!("Nenhum monitor ativo encontrado, centralizando na tela principal");
    window.center().map_err(|e| format!("Falha ao centralizar a janela: {}", e))
}

// ✅ NOVA FUNÇÃO: Obtém posição global do cursor no macOS
#[cfg(target_os = "macos")]
fn get_global_cursor_position() -> Result<PhysicalPosition<f64>, String> {
    use cocoa::appkit::{NSEvent, NSScreen};
    use cocoa::foundation::NSPoint;
    use cocoa::base::nil;

    unsafe {
        // Corrigir: NSEvent::mouseLocation precisa de parâmetro
        let location: NSPoint = NSEvent::mouseLocation(nil);
        
        // No macOS, precisamos converter o sistema de coordenadas
        // NSEvent::mouseLocation() retorna coordenadas com origem no canto inferior esquerdo
        // Tauri usa coordenadas com origem no canto superior esquerdo
        
        // Obter a altura da tela principal para conversão
        let main_screen = NSScreen::mainScreen(nil);
        if main_screen.is_null() {
            return Err("Não foi possível obter tela principal".to_string());
        }
        
        let screen_frame = NSScreen::frame(main_screen);
        let screen_height = screen_frame.size.height;
        
        // Converter coordenada Y (inverter)
        let converted_y = screen_height - location.y;
        
        debug!("Cursor NSEvent: x={}, y={} | Convertido: x={}, y={}", 
               location.x, location.y, location.x, converted_y);
        
        Ok(PhysicalPosition::new(location.x, converted_y))
    }
}

// ✅ VERSÃO PARA OUTRAS PLATAFORMAS (fallback)
#[cfg(not(target_os = "macos"))]
fn get_global_cursor_position() -> Result<PhysicalPosition<f64>, String> {
    // Para outras plataformas, retorna posição padrão (centro da tela principal)
    Err("Global cursor position not implemented for this platform".to_string())
}

// ✅ CONFIGURAÇÃO GLOBAL: Faz janela aparecer em TODOS os espaços do macOS (como Spotlight)
#[cfg(target_os = "macos")]
fn configure_search_window_for_global_access<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    use cocoa::appkit::NSWindowCollectionBehavior;
    use cocoa::base::id;

    unsafe {
        if let Ok(ns_window) = window.ns_window() {
            let ns_window = ns_window as id;
            
            // ✅ CONFIGURAÇÃO CRUCIAL: Aparece em todos os espaços + não afeta Mission Control
            let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace;
            
            // Aplicar comportamento usando objc
            use objc::{msg_send, sel, sel_impl};
            let _: () = msg_send![ns_window, setCollectionBehavior: behavior];
            
            // Nível alto mas não extremo (acima de apps normais, abaixo de notificações de sistema)
            let _: () = msg_send![ns_window, setLevel: 200];
            
            info!("Janela configurada para aparecer em todos os espaços do macOS");
        }
    }

    Ok(())
}

// ✅ CONFIGURAÇÃO GLOBAL para Windows
#[cfg(target_os = "windows")]
fn configure_search_window_for_global_access<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE
    };
    use windows::Win32::Foundation::HWND;

    unsafe {
        if let Ok(hwnd) = window.hwnd() {
            let hwnd = HWND(hwnd.0);
            
            // Configura como topmost para aparecer sobre todas as outras janelas
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE,
            );
            
            info!("Janela configurada para aparecer globalmente (Windows)");
        }
    }

    Ok(())
}

// ✅ CONFIGURAÇÃO GLOBAL para outras plataformas
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn configure_search_window_for_global_access<R: Runtime>(_window: &WebviewWindow<R>) -> Result<(), String> {
    info!("Configuração global usando métodos padrão do Tauri");
    Ok(())
}

// ✅ VERSÃO SEGURA: Função específica para janela de busca aparecer em primeiro plano
#[cfg(target_os = "macos")]
fn bring_search_window_to_front<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    use cocoa::appkit::{NSApp, NSApplication};
    use cocoa::base::{id, YES};
    use objc::{msg_send, sel, sel_impl};

    unsafe {
        // Ativa aplicação de forma suave (sem interferir com outras janelas)
        let app: id = NSApp();
        let _: () = msg_send![app, activateIgnoringOtherApps: YES];

        // Configuração mínima e segura para janela de busca
        if let Ok(ns_window) = window.ns_window() {
            let ns_window = ns_window as id;
            
            // Nível moderado - suficiente para aparecer sobre apps mas não extremo
            let _: () = msg_send![ns_window, setLevel: 100]; 
            
            // Traz para frente de forma suave
            let _: () = msg_send![ns_window, orderFront: ns_window];
        }
    }

    debug!("Search window brought to front safely");
    Ok(())
}

// Função original comentada para referência (causava crash)
/*
#[cfg(target_os = "macos")]
fn bring_window_to_front<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy, NSWindow, NSWindowCollectionBehavior};
    use cocoa::base::{id, nil, YES, NO};
    use objc::{msg_send, sel, sel_impl};

    unsafe {
        // Ativa a aplicação ignorando outras apps
        let app: id = NSApp();
        let _: () = msg_send![app, activateIgnoringOtherApps: YES];

        // Configura o comportamento da janela para aparecer em todos os espaços
        if let Ok(ns_window) = window.ns_window() {
            let ns_window = ns_window as id;

            // Configurações para sempre aparecer à frente
            let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle;

            let _: () = msg_send![ns_window, setCollectionBehavior: behavior];
            let _: () = msg_send![ns_window, setLevel: 1000]; // Nível muito alto
            let _: () = msg_send![ns_window, orderFrontRegardless];
        }
    }

    Ok(())
}
*/

// ✅ VERSÃO SEGURA para Windows
#[cfg(target_os = "windows")]
fn bring_search_window_to_front<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, SetForegroundWindow, BringWindowToTop,
        HWND_TOP, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW
    };
    use windows::Win32::Foundation::HWND;

    unsafe {
        if let Ok(hwnd) = window.hwnd() {
            let hwnd = HWND(hwnd.0);

            // Versão mais suave - HWND_TOP em vez de HWND_TOPMOST
            let _ = SetWindowPos(
                hwnd,
                HWND_TOP,
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
            );

            // Traz para frente de forma suave
            let _ = BringWindowToTop(hwnd);
            let _ = SetForegroundWindow(hwnd);
        }
    }

    debug!("Search window brought to front safely (Windows)");
    Ok(())
}

// ✅ VERSÃO SEGURA para outras plataformas
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn bring_search_window_to_front<R: Runtime>(_window: &WebviewWindow<R>) -> Result<(), String> {
    // Para outras plataformas, os métodos nativos do Tauri são suficientes
    debug!("Search window brought to front using Tauri defaults");
    Ok(())
}

// Funções originais comentadas para referência
/*
#[cfg(target_os = "windows")]
fn bring_window_to_front<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowPos, SetForegroundWindow, BringWindowToTop,
        HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW
    };
    use windows::Win32::Foundation::HWND;

    unsafe {
        if let Ok(hwnd) = window.hwnd() {
            let hwnd = HWND(hwnd.0);

            // Define como topmost com flag de mostrar
            let _ = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
            );

            // Traz para frente e define como foreground
            let _ = BringWindowToTop(hwnd);
            let _ = SetForegroundWindow(hwnd);
        }
    }

    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn bring_window_to_front<R: Runtime>(_window: &WebviewWindow<R>) -> Result<(), String> {
    // Para outras plataformas, o comportamento padrão do Tauri deve ser suficiente
    Ok(())
}
*/

async fn get_window_state<R: Runtime>(window: &WebviewWindow<R>) -> Result<WindowState, String> {
    let visible = window.is_visible()
        .map_err(|e| format!("Failed to get visibility: {}", e))?;

    let focused = window.is_focused()
        .map_err(|e| format!("Failed to get focus state: {}", e))?;

    Ok(WindowState {
        visible,
        focused,
        always_on_top: true,
    })
}

// Função para configurar eventos da janela (auto-hide ao perder foco)
#[command]
pub async fn setup_search_window_events<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    let window = get_search_window(&app_handle)?;

    // Clone para usar no closure (comentado junto com o código que o usava)
    let _window_clone = window.clone();

    // ✅ CORREÇÃO: Auto-hide agora é gerenciado apenas pelo useGlobalShortcut hook
    // Comentado para evitar conflitos com o sistema de auto-hide do frontend
    /*
    // Configura listener para eventos da janela
    window.on_window_event(move |event| {
        match event {
            tauri::WindowEvent::Focused(focused) => {
                if !*focused {
                    // Janela perdeu o foco - esconde após pequeno delay para evitar flicker
                    let window = window_clone.clone();
                    tauri::async_runtime::spawn(async move {
                        // Pequeno delay para evitar esconder muito rapidamente
                        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

                        // Verifica se ainda não tem foco antes de esconder
                        if let Ok(focused) = window.is_focused() {
                            if !focused {
                                if let Err(e) = hide_search_window_internal(&window).await {
                                    eprintln!("Failed to auto-hide window: {}", e);
                                }
                                unsafe { SEARCH_WINDOW_VISIBLE = false; }
                            }
                        }
                    });
                }
            }
            _ => {}
        }
    });
    */

    Ok(())
}
