// /Users/rafaelreis/R5 Flowlight/src-tauri/src/commands/settings/commands.rs
// VERSÃO CORRIGIDA

use tauri::{AppHandle, Manager, Runtime};
use log::{info, error};

// ✅ CORREÇÃO: Importar das fontes corretas, não de commands::settings
use crate::commands::settings::{load_settings, save_settings, reset_settings, update_autostart};
use crate::commands::settings::AppSettings;

#[tauri::command]
pub async fn load_settings_cmd(app: AppHandle) -> Result<AppSettings, String> {
    info!("Carregando configurações...");
    load_settings(&app.path()).map_err(|e| {
        error!("Erro ao carregar configurações: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub async fn save_settings_cmd(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    info!("Salvando configurações...");
    save_settings(&app.path(), &settings).map_err(|e| {
        error!("Erro ao salvar configurações: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub async fn reset_settings_cmd(app: AppHandle) -> Result<(), String> {
    info!("Reiniciando configurações para os padrões...");
    reset_settings(&app.path()).map_err(|e| {
        error!("Erro ao reiniciar configurações: {}", e);
        e.to_string()
    })
}

#[tauri::command]
pub async fn update_autostart_cmd(app: AppHandle, start_with_system: bool) -> Result<(), String> {
    info!("Atualizando configuração de inicialização automática para: {}", start_with_system);
    update_autostart(&app.path(), start_with_system).map_err(|e| {
        error!("Erro ao atualizar configuração de inicialização automática: {}", e);
        e.to_string()
    })
}
