use postgrest::Postgrest;
use std::env;
use std::sync::OnceLock;

static SUPABASE_CLIENT: OnceLock<Postgrest> = OnceLock::new();

pub fn get_supabase_client() -> &'static Postgrest {
    SUPABASE_CLIENT.get_or_init(|| {
        let supabase_url = env::var("SUPABASE_URL")
            .expect("SUPABASE_URL must be set in .env file");
        
        // Usamos a SERVICE_KEY para o backend ter permissões de administrador.
        let supabase_key = env::var("SUPABASE_SERVICE_KEY")
            .expect("SUPABASE_SERVICE_KEY must be set in .env file");

        // O cliente é configurado com a URL e a chave de serviço como header de autorização.
        Postgrest::new(format!("{}/rest/v1", supabase_url))
            .insert_header("apikey", supabase_key.clone())
            .insert_header("Authorization", format!("Bearer {}", supabase_key))
    })
}
