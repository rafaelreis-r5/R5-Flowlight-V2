//! Logger utility
//! 
//! Configuração do sistema de logging da aplicação

use log::LevelFilter;
use env_logger::Builder;
use std::io::Write;

/// Inicializa o sistema de logging
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = Builder::from_default_env();
    
    builder
        .filter_level(if cfg!(debug_assertions) {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
    
    Ok(())
}