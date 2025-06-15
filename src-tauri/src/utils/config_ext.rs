//! Extensões para a configuração da aplicação

use std::path::Path;
use super::config::AppConfig;

/// Extensão para AppConfig com métodos auxiliares para verificação de indexação
pub trait AppConfigExt {
    /// Verifica se um arquivo deve ser indexado com base nas configurações
    fn should_index_file(&self, path: &Path) -> bool;
    
    /// Verifica se um diretório deve ser indexado com base nas configurações
    fn should_index_dir(&self, path: &Path) -> bool;
    
    /// Verifica se um caminho deve ser ignorado com base nas configurações
    fn should_ignore_path(&self, path: &Path) -> bool;
    
    /// Verifica se uma extensão de arquivo deve ser indexada
    fn should_index_extension(&self, extension: &str) -> bool;
}

impl AppConfigExt for AppConfig {
    fn should_index_file(&self, path: &Path) -> bool {
        // Verificar se o caminho está na lista de ignorados
        if self.should_ignore_path(path) {
            return false;
        }
        
        // Verificar extensão do arquivo
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if !self.should_index_extension(ext) {
                return false;
            }
        }
        
        // Verificar tamanho máximo do arquivo (se aplicável)
        if let Ok(metadata) = std::fs::metadata(path) {
            let max_size_bytes = self.search.max_file_size_mb * 1024 * 1024;
            if metadata.len() > max_size_bytes {
                return false;
            }
        }
        
        true
    }
    
    fn should_index_dir(&self, path: &Path) -> bool {
        // Verificar se o caminho está na lista de ignorados
        if self.should_ignore_path(path) {
            return false;
        }
        
        // Verificar se o diretório está na lista de caminhos indexados
        for indexed_path in &self.search.indexed_paths {
            if path.starts_with(indexed_path) {
                return true;
            }
        }
        
        false
    }
    
    fn should_ignore_path(&self, path: &Path) -> bool {
        // Verificar se o caminho está na lista de ignorados
        for ignored_path in &self.search.ignored_paths {
            if path.starts_with(ignored_path) {
                return true;
            }
        }
        
        // Verificar se é um diretório oculto (começa com .)
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.starts_with('.') && file_name != "." && !self.search.include_hidden {
                return true;
            }
            
            // Ignorar diretórios do sistema
            let ignored_dirs = ["node_modules", "target", "dist", "build", ".git"];
            if ignored_dirs.contains(&file_name) {
                return true;
            }
        }
        
        false
    }
    
    fn should_index_extension(&self, extension: &str) -> bool {
        // Se não houver extensões especificadas, indexa todas
        if self.search.indexed_extensions.is_empty() {
            return true;
        }
        
        // Verificar se a extensão está na lista de extensões indexadas
        self.search.indexed_extensions
            .iter()
            .any(|ext| ext.eq_ignore_ascii_case(extension))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    fn create_test_config() -> AppConfig {
        let mut config = AppConfig::default();
        config.search.indexed_paths = vec![
            Path::new("/home/user/documents").to_path_buf(),
            Path::new("/home/user/projects").to_path_buf(),
        ];
        config.search.ignored_paths = vec![
            Path::new("/home/user/documents/private").to_path_buf(),
            Path::new("/home/user/projects/temp").to_path_buf(),
        ];
        config.search.indexed_extensions = vec![
            "txt".to_string(),
            "rs".to_string(),
            "md".to_string(),
        ];
        config.search.max_file_size_mb = 10; // 10MB
        config.search.include_hidden = false;
        config
    }
    
    #[test]
    fn test_should_index_file() {
        let config = create_test_config();
        
        // Arquivos com extensões suportadas
        assert!(config.should_index_file(Path::new("/home/user/documents/note.txt")));
        assert!(config.should_index_file(Path::new("/home/user/projects/app/src/main.rs")));
        
        // Arquivos com extensões não suportadas
        assert!(!config.should_index_file(Path::new("/home/user/documents/image.png")));
        
        // Arquivos em pastas ignoradas
        assert!(!config.should_index_file(Path::new("/home/user/documents/private/secret.txt")));
        
        // Arquivos ocultos
        assert!(!config.should_index_file(Path::new("/home/user/documents/.hidden.txt")));
    }
    
    #[test]
    fn test_should_index_dir() {
        let config = create_test_config();
        
        // Diretórios na lista de indexados
        assert!(config.should_index_dir(Path::new("/home/user/documents")));
        assert!(config.should_index_dir(Path::new("/home/user/projects/myapp")));
        
        // Diretórios fora da lista de indexados
        assert!(!config.should_index_dir(Path::new("/home/user/downloads")));
        
        // Diretórios na lista de ignorados
        assert!(!config.should_index_dir(Path::new("/home/user/documents/private")));
        
        // Diretórios ocultos
        assert!(!config.should_index_dir(Path::new("/home/user/.config")));
    }
    
    #[test]
    fn test_should_ignore_path() {
        let config = create_test_config();
        
        // Caminhos na lista de ignorados
        assert!(config.should_ignore_path(Path::new("/home/user/documents/private")));
        assert!(config.should_ignore_path(Path::new("/home/user/projects/temp/file.txt")));
        
        // Caminhos não ignorados
        assert!(!config.should_ignore_path(Path::new("/home/user/documents")));
        
        // Caminhos ocultos
        assert!(config.should_ignore_path(Path::new("/home/user/.hidden")));
        
        // Diretórios do sistema
        assert!(config.should_ignore_path(Path::new("/home/user/projects/node_modules")));
    }
    
    #[test]
    fn test_should_index_extension() {
        let config = create_test_config();
        
        // Extensões suportadas
        assert!(config.should_index_extension("txt"));
        assert!(config.should_index_extension("rs"));
        assert!(config.should_index_extension("md"));
        
        // Extensões não suportadas
        assert!(!config.should_index_extension("png"));
        assert!(!config.should_index_extension("jpg"));
        
        // Verificação case-insensitive
        assert!(config.should_index_extension("TXT"));
        assert!(config.should_index_extension("Rs"));
    }
}
