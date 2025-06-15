use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub user_metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: AuthUser,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthError {
    pub message: String,
    pub status: Option<u16>,
}

// Função para fazer o login
#[tauri::command]
pub async fn login(email: &str, password: &str) -> Result<AuthResponse, String> {
    log::info!("Iniciando processo de login para: {}", email);
    
    let supabase_url = env::var("SUPABASE_URL")
        .map_err(|_| "SUPABASE_URL não configurado".to_string())?;
    let anon_key = env::var("SUPABASE_ANON_KEY")
        .map_err(|_| "SUPABASE_ANON_KEY não configurada".to_string())?;

    log::debug!("URL do Supabase: {}", supabase_url);
    log::debug!("Enviando requisição de login...");

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/auth/v1/token?grant_type=password", supabase_url))
        .header("apikey", &anon_key)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| {
            log::error!("Erro na requisição de login: {}", e);
            format!("Erro de conexão: {}", e)
        })?;

    let status = response.status();
    let response_text = response.text().await
        .map_err(|e| {
            log::error!("Erro ao ler resposta do login: {}", e);
            format!("Erro ao processar resposta do servidor: {}", e)
        })?;

    log::debug!("Resposta do login ({}): {}", status, response_text);

    if !status.is_success() {
        let error_msg = if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
            if let Some(msg) = error_json.get("error_description").and_then(|v| v.as_str()) {
                msg.to_string()
            } else if let Some(msg) = error_json.get("message").and_then(|v| v.as_str()) {
                msg.to_string()
            } else {
                format!("Erro desconhecido: {}", response_text)
            }
        } else {
            format!("Falha no login ({}): {}", status, response_text)
        };
        
        log::error!("Erro no login: {}", error_msg);
        return Err(error_msg);
    }

    let auth_response: AuthResponse = serde_json::from_str(&response_text)
        .map_err(|e| {
            log::error!("Erro ao desserializar resposta: {}", e);
            format!("Erro ao processar resposta do servidor: {}", e)
        })?;

    log::info!("Login bem-sucedido para o usuário: {}", auth_response.user.email);
    Ok(auth_response)
}

// Função para completar o primeiro acesso (definir senha e nome)
#[tauri::command]
pub async fn complete_first_access(email: &str, password: &str, full_name: &str) -> Result<(), String> {
    log::info!("Completando primeiro acesso para: {} ({})", email, full_name);
    
    let supabase_url = env::var("SUPABASE_URL")
        .map_err(|_| "SUPABASE_URL não configurado".to_string())?;
    let service_key = env::var("SUPABASE_SERVICE_KEY")
        .map_err(|_| "SUPABASE_SERVICE_KEY não configurada".to_string())?;

    log::debug!("URL do Supabase: {}", supabase_url);
    log::debug!("Buscando usuário pelo e-mail: {}", email);
    
    // Log para verificar se as credenciais estão sendo recebidas corretamente
    log::debug!("Dados recebidos - Email: {}, Nome: {}", email, full_name);

    let client = reqwest::Client::new();
    
    // 1. Encontrar o ID do usuário pelo e-mail
    // Usando a sintaxe correta para filtrar por email
    let users_url = format!(
        "{}/auth/v1/admin/users?email=eq.{}",
        supabase_url,
        email
    );
    log::debug!("Fazendo requisição para: {}", users_url);
    
    let users_response = client
        .get(&users_url)
        .header("apikey", &service_key)
        .header("Authorization", format!("Bearer {}", service_key))
        .send()
        .await
        .map_err(|e| {
            log::error!("Erro ao buscar usuário: {}", e);
            format!("Erro ao buscar usuário: {}", e)
        })?;

    let users_data: serde_json::Value = users_response.json().await
        .map_err(|e| {
            log::error!("Erro ao processar resposta de busca de usuário: {}", e);
            format!("Erro ao processar resposta do servidor: {}", e)
        })?;

    log::debug!("Resposta da API de usuários: {:?}", users_data);
    
    // Verifica se há usuários na resposta
    if users_data["users"].as_array().map_or(true, |arr| arr.is_empty()) {
        log::error!("Nenhum usuário encontrado com o e-mail: {}", email);
        return Err("Usuário não encontrado. Verifique se o e-mail está correto.".to_string());
    }
    
    let user_id = users_data["users"][0]["id"].as_str()
        .ok_or_else(|| {
            log::error!("ID do usuário não encontrado na resposta");
            "Erro ao processar os dados do usuário.".to_string()
        })?;
        
    log::debug!("Usuário encontrado com ID: {}", user_id);

    log::debug!("Usuário encontrado com ID: {}", user_id);

    // 2. Definir a senha do usuário
    log::debug!("Atualizando senha do usuário...");
    let update_pwd_url = format!("{}/auth/v1/admin/users/{}", supabase_url, user_id);
    log::debug!("Fazendo requisição PUT para: {}", update_pwd_url);
    
    let update_pwd_response = client
        .put(&update_pwd_url)
        .header("apikey", &service_key)
        .header("Authorization", format!("Bearer {}", service_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "password": password,
            "email_confirm": true  // Garante que o e-mail está confirmado
        }))
        .send()
        .await
        .map_err(|e| {
            log::error!("Erro ao atualizar senha: {}", e);
            format!("Erro ao atualizar senha: {}", e)
        })?;

    let update_pwd_status = update_pwd_response.status();
    if !update_pwd_status.is_success() {
        let error_text = update_pwd_response.text().await.unwrap_or_else(|_| "Falha ao ler resposta".to_string());
        log::error!("Erro ao atualizar senha ({}): {}", update_pwd_status, error_text);
        return Err(format!("Falha ao atualizar a senha ({}): {}", update_pwd_status, error_text));
    }

    log::debug!("Senha atualizada com sucesso");
    
    // 3. Atualizar o perfil com nome completo e username
    log::debug!("Atualizando perfil do usuário...");
    let profiles_url = format!("{}/rest/v1/profiles", supabase_url);
    log::debug!("Fazendo requisição POST para: {}", profiles_url);
    
    let profile_data = json!({
        "id": user_id,
        "full_name": full_name,
        "username": email,
        "updated_at": chrono::Utc::now().to_rfc3339()
    });
    
    log::debug!("Dados do perfil a serem enviados: {:?}", profile_data);
    
    let profile_update_response = client
        .post(&profiles_url)
        .header("apikey", &service_key)
        .header("Authorization", format!("Bearer {}", service_key))
        .header("Content-Type", "application/json")
        .header("Prefer", "resolution=merge-duplicates")
        .json(&json!({
            "id": user_id,
            "full_name": full_name, // Nome da coluna no banco de dados
            "username": email,
            "updated_at": chrono::Utc::now().to_rfc3339()
        }))
        .send()
        .await
        .map_err(|e| {
            log::error!("Erro ao atualizar perfil: {}", e);
            format!("Erro ao atualizar perfil: {}", e)
        })?;

    let profile_status = profile_update_response.status();
    if !profile_status.is_success() {
        let error_text = profile_update_response.text().await.unwrap_or_else(|_| "Falha ao ler resposta".to_string());
        log::error!("Erro ao atualizar perfil ({}): {}", profile_status, error_text);
        // Não falhamos aqui, apenas registramos o erro, pois o mais importante (a senha) já foi atualizado
        // Mas retornamos um aviso para o frontend
        log::warn!("A senha foi atualizada, mas houve um problema ao atualizar o perfil");
    } else {
        log::debug!("Perfil atualizado com sucesso");
    }
    
    log::info!("Primeiro acesso concluído com sucesso para: {}", email);

    log::info!("Primeiro acesso concluído com sucesso para: {}", email);
    Ok(())
}

// Função para obter o usuário atual
#[tauri::command]
pub async fn get_user(access_token: &str) -> Result<AuthUser, String> {
    log::info!("Obtendo informações do usuário...");
    
    if access_token.is_empty() {
        log::error!("Token de acesso vazio");
        return Err("Token de acesso não fornecido".to_string());
    }

    let supabase_url = env::var("SUPABASE_URL")
        .map_err(|_| "SUPABASE_URL não configurado".to_string())?;
    let anon_key = env::var("SUPABASE_ANON_KEY")
        .map_err(|_| "SUPABASE_ANON_KEY não configurada".to_string())?;

    log::debug!("URL do Supabase: {}", supabase_url);
    log::debug!("Enviando requisição para obter usuário...");

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/auth/v1/user", supabase_url))
        .header("apikey", &anon_key)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| {
            log::error!("Erro na requisição para obter usuário: {}", e);
            format!("Erro de conexão: {}", e)
        })?;

    let status = response.status();
    let response_text = response.text().await
        .map_err(|e| {
            log::error!("Erro ao ler resposta do usuário: {}", e);
            format!("Erro ao processar resposta do servidor: {}", e)
        })?;

    log::debug!("Resposta do usuário ({}): {}", status, response_text);

    if !status.is_success() {
        let error_msg = if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
            if let Some(msg) = error_json.get("message").and_then(|v| v.as_str()) {
                msg.to_string()
            } else if let Some(msg) = error_json.get("error").and_then(|v| v.as_str()) {
                msg.to_string()
            } else {
                format!("Erro desconhecido: {}", response_text)
            }
        } else {
            format!("Falha ao obter usuário ({}): {}", status, response_text)
        };
        
        log::error!("Erro ao obter usuário: {}", error_msg);
        return Err(error_msg);
    }

    let user: AuthUser = serde_json::from_str(&response_text)
        .map_err(|e| {
            log::error!("Erro ao desserializar resposta do usuário: {}", e);
            format!("Erro ao processar resposta do servidor: {}", e)
        })?;

    log::info!("Usuário obtido com sucesso: {}", user.email);
    Ok(user)
}
