//! Search API handlers
//! 
//! Handlers para os endpoints de busca

use serde::{Deserialize, Serialize};
use log::info;

/// Resultado de busca de arquivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub path: String,
    pub content_preview: String,
    pub score: f32,
    pub file_type: String,
    pub size: u64,
    pub modified: chrono::DateTime<chrono::Utc>,
}

/// Resultado de busca de aplicativo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppResult {
    pub name: String,
    pub path: String,
    pub icon: Option<String>,
    pub version: Option<String>,
    pub bundle_id: Option<String>,
}

/// Handler para busca de arquivos
pub async fn search_files_handler(query: String) -> Result<Vec<SearchResult>, String> {
    info!("API: Searching files for query: {}", query);
    
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }
    
    // Busca real de arquivos no sistema
    use std::path::Path;
    use walkdir::WalkDir;
    use std::fs;
    
    let mut results = Vec::new();
    let home_dir = dirs::home_dir().unwrap_or_else(|| Path::new("/").to_path_buf());
    
    // Diretórios comuns para buscar
    let search_dirs = vec![
        home_dir.join("Documents"),
        home_dir.join("Desktop"),
        home_dir.join("Downloads"),
    ];
    
    for search_dir in search_dirs {
        if !search_dir.exists() {
            continue;
        }
        
        let walker = WalkDir::new(&search_dir)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .take(10); // Limite para performance
            
        for entry in walker {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            let query_lower = query.to_lowercase();
            
            if file_name.contains(&query_lower) {
                let path = entry.path();
                let metadata = fs::metadata(path).ok();
                
                let file_type = path.extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                let modified = metadata
                    .and_then(|m| m.modified().ok())
                    .and_then(|t| chrono::DateTime::from(t).checked_sub_signed(chrono::Duration::seconds(0)))
                    .unwrap_or_else(chrono::Utc::now);
                
                // Calcular score baseado na similaridade
                let score = if file_name == query_lower {
                    1.0
                } else if file_name.starts_with(&query_lower) {
                    0.9
                } else {
                    0.7
                };
                
                results.push(SearchResult {
                    title: path.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Unknown".to_string()),
                    path: path.to_string_lossy().to_string(),
                    content_preview: format!("Arquivo {} encontrado", file_type.to_uppercase()),
                    score,
                    file_type,
                    size,
                    modified,
                });
                
                if results.len() >= 10 {
                    break;
                }
            }
        }
        
        if results.len() >= 10 {
            break;
        }
    }
    
    // Ordenar por score
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    
    Ok(results)
}

/// Handler para busca de aplicativos
pub async fn search_apps_handler(query: String) -> Result<Vec<AppResult>, String> {
    info!("API: Searching apps for query: {}", query);
    
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }
    
    // Busca real de aplicativos no macOS
    use std::path::Path;
    use walkdir::WalkDir;
    
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();
    
    // Diretórios de aplicativos no macOS
    let home_apps = format!("{}/Applications", dirs::home_dir()
        .map(|d| d.to_string_lossy().to_string())
        .unwrap_or_else(|| "/Users".to_string()));
        
    let app_dirs = vec![
        "/Applications".to_string(),
        "/System/Applications".to_string(),
        home_apps,
    ];
    
    for app_dir in &app_dirs {
        let app_path = Path::new(app_dir);
        if !app_path.exists() {
            continue;
        }
        
        let walker = WalkDir::new(app_path)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_dir() &&
                e.path().extension().and_then(|ext| ext.to_str()) == Some("app")
            })
            .take(20);
            
        for entry in walker {
            let app_name = entry.file_name().to_string_lossy();
            let clean_name = app_name.trim_end_matches(".app");
            
            if clean_name.to_lowercase().contains(&query_lower) {
                let path = entry.path().to_string_lossy().to_string();
                
                // Tentar extrair informações do Info.plist
                let info_plist_path = entry.path().join("Contents/Info.plist");
                let (bundle_id, version) = if info_plist_path.exists() {
                    // Implementação simples - em produção usaria plist parser
                    (Some("com.app.unknown".to_string()), Some("1.0".to_string()))
                } else {
                    (None, None)
                };
                
                results.push(AppResult {
                    name: clean_name.to_string(),
                    path,
                    icon: Some(format!("{}/Contents/Resources/AppIcon.icns", entry.path().display())),
                    version,
                    bundle_id,
                });
                
                if results.len() >= 10 {
                    break;
                }
            }
        }
        
        if results.len() >= 10 {
            break;
        }
    }
    
    // Ordenar alfabeticamente
    results.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(results)
}