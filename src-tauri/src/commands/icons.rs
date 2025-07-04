use tauri::command;
use std::path::Path;
use std::collections::HashMap;
use std::sync::Mutex;

// Estruturas de dados
#[derive(serde::Serialize, Clone)]
pub struct IconResult {
    pub success: bool,
    pub data: Option<String>,
    pub error: Option<String>,
    pub size: u32,
    pub path: String,
}

#[derive(serde::Serialize)]
pub struct BatchIconResult {
    pub icons: Vec<IconResult>,
    pub total_requested: usize,
    pub successful: usize,
    pub failed: usize,
}

// Cache global em memória
lazy_static::lazy_static! {
    static ref ICON_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// Função principal para obter ícone individual
#[command]
pub async fn get_file_icon(path: String, size: Option<u32>) -> IconResult {
    let icon_size = size.unwrap_or(32);
    let cache_key = format!("{}_{}", path, icon_size);
    
    // Verifica cache primeiro
    {
        let cache = ICON_CACHE.lock().unwrap();
        if let Some(cached_data) = cache.get(&cache_key) {
            return IconResult {
                success: true,
                data: Some(cached_data.clone()),
                error: None,
                size: icon_size,
                path: path.clone(),
            };
        }
    }
    
    // Busca o ícone se não estiver em cache
    match get_icon_impl(&path, icon_size).await {
        Ok(base64_data) => {
            // Armazena no cache
            {
                let mut cache = ICON_CACHE.lock().unwrap();
                cache.insert(cache_key, base64_data.clone());
            }
            
            IconResult {
                success: true,
                data: Some(base64_data),
                error: None,
                size: icon_size,
                path,
            }
        },
        Err(e) => IconResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
            size: icon_size,
            path,
        }
    }
}

// Função para obter múltiplos ícones (otimizada para listas)
#[command]
pub async fn get_file_icons_batch(paths: Vec<String>, size: Option<u32>) -> BatchIconResult {
    let icon_size = size.unwrap_or(32);
    let total_requested = paths.len();
    
    let tasks: Vec<_> = paths.into_iter()
        .map(|path| get_file_icon(path, Some(icon_size)))
        .collect();
    
    let results = futures::future::join_all(tasks).await;
    let successful = results.iter().filter(|r| r.success).count();
    let failed = total_requested - successful;
    
    BatchIconResult {
        icons: results,
        total_requested,
        successful,
        failed,
    }
}

// Implementação específica para macOS
#[cfg(target_os = "macos")]
async fn get_icon_impl(path: &str, size: u32) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use cocoa::base::{id, nil};
    use cocoa::foundation::{NSString, NSData};
    use cocoa::appkit::NSImage;
    use objc::{msg_send, sel, sel_impl, class};
    
    let path_owned = path.to_string();
    
    tokio::task::spawn_blocking(move || {
        unsafe {
            // Verifica se o arquivo existe
            if !Path::new(&path_owned).exists() {
                return Err(format!("File does not exist: {}", path_owned).into());
            }

            // NSWorkspace compartilhado
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            
            // Converte path para NSString
            let ns_path = NSString::alloc(nil).init_str(&path_owned);
            
            // Obtém o ícone do arquivo
            let image: id = msg_send![workspace, iconForFile: ns_path];
            if image == nil {
                return Err("Could not get icon for file".into());
            }

            // Define o tamanho desejado
            let size_obj = cocoa::foundation::NSSize::new(size as f64, size as f64);
            let _: () = msg_send![image, setSize: size_obj];

            // Por enquanto, retornamos um ícone placeholder simples como base64
            // TODO: Implementar conversão completa NSImage -> PNG -> Base64
            // Esta é uma imagem PNG simples de 16x16 de um arquivo genérico
            let placeholder_icon = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAAdgAAAHYBTnsmCAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAAESSURBVDiNpZM9SwNBEIafgwQLwcJCG1sLG1sLwcJCG2ux0NZCsLDQQrCw0EKwsNDCQltbC8FCG1sLwcJCG2ux0NZCsLDQQrCw0EKwsNDCQltbC8HCQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sdDGQhsLbSy0sQAAAABJRU5ErkJggg==";
            Ok(placeholder_icon.to_string())
        }
    }).await?
}

// Implementação específica para Windows
#[cfg(target_os = "windows")]
async fn get_icon_impl(path: &str, size: u32) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use windows::{
        core::PCWSTR,
        Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON, SHGFI_SMALLICON},
        Win32::UI::WindowsAndMessaging::DestroyIcon,
        Win32::Foundation::MAX_PATH,
        Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES,
    };
    
    let path_owned = path.to_string();
    
    tokio::task::spawn_blocking(move || {
        unsafe {
            // Verifica se arquivo existe
            if !Path::new(&path_owned).exists() {
                return Err(format!("File does not exist: {}", path_owned).into());
            }

            let wide_path: Vec<u16> = path_owned.encode_utf16().chain(std::iter::once(0)).collect();
            let mut file_info = SHFILEINFOW::default();
            
            let flags = SHGFI_ICON | if size > 16 { SHGFI_LARGEICON } else { SHGFI_SMALLICON };
            
            let result = SHGetFileInfoW(
                PCWSTR(wide_path.as_ptr()),
                FILE_FLAGS_AND_ATTRIBUTES(0),
                Some(&mut file_info),
                std::mem::size_of::<SHFILEINFOW>() as u32,
                flags,
            );
            
            if result == 0 || file_info.hIcon.is_invalid() {
                return Err("Could not get file icon".into());
            }

            // TODO: Implementar conversão HICON para PNG/Base64
            // Por enquanto, cleanup e retorna erro
            let _ = DestroyIcon(file_info.hIcon);
            
            // Placeholder - implementar conversão HICON -> PNG -> Base64
            Err("Windows icon conversion not yet implemented".into())
        }
    }).await?
}

// Fallback para sistemas não suportados
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
async fn get_icon_impl(_path: &str, _size: u32) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    Err("Unsupported platform".into())
}

// Função para limpar cache (útil para desenvolvimento)
#[command]
pub fn clear_icon_cache() -> bool {
    match ICON_CACHE.lock() {
        Ok(mut cache) => {
            cache.clear();
            true
        },
        Err(_) => false,
    }
}

// Função para obter estatísticas do cache
#[command]
pub fn get_cache_stats() -> serde_json::Value {
    match ICON_CACHE.lock() {
        Ok(cache) => serde_json::json!({
            "size": cache.len(),
            "keys": cache.keys().collect::<Vec<_>>()
        }),
        Err(_) => serde_json::json!({
            "error": "Could not access cache"
        }),
    }
}