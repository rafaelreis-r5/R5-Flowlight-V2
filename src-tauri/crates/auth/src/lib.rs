//! # Authentication Module for R5 Flowlight
//! 
//! This module provides JWT-based authentication, password hashing, and user management
//! for the R5 Flowlight application.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

#[macro_use]
extern crate lazy_static;

use log::{info, error};
use std::sync::RwLock;
use std::env;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod rate_limit;

lazy_static! {
    /// In-memory user store (replace with a database in production)
    pub static ref USERS: RwLock<Vec<User>> = RwLock::new(Vec::new());
    
    /// JWT secret key (should be set via JWT_SECRET environment variable)
    pub static ref JWT_SECRET: String = {
        env::var("JWT_SECRET").unwrap_or_else(|_| {
            let default_secret = "your_jwt_secret_key_here".to_string();
            error!("JWT_SECRET not set, using default. This is not secure for production!");
            default_secret
        })
    };
    
    /// JWT token duration in seconds (1 hour by default)
    pub static ref TOKEN_DURATION_SECONDS: i64 = {
        env::var("TOKEN_DURATION_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3600) // Default: 1 hour
    };
    
    /// Refresh token duration in seconds (7 days by default)
    pub static ref REFRESH_TOKEN_DURATION_SECONDS: i64 = {
        env::var("REFRESH_TOKEN_DURATION_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(604800) // Default: 7 days
    };
}

/// Initialize the auth module
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing auth module...");
    
    // Create a default admin user if none exists
    let admin_username = env::var("DEFAULT_ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let admin_password = env::var("DEFAULT_ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".to_string());
    
    let users = USERS.read().map_err(|e| format!("Failed to read users: {}", e))?;
    let admin_exists = users.iter().any(|u| u.role == UserRole::Admin);
    
    if !admin_exists {
        drop(users); // Release the read lock
        
        // Create the admin user
        match create_user_internal(
            admin_username.clone(),
            admin_password,
            UserRole::Admin,
        ).await {
            Ok(_) => info!("Created default admin user: {}", admin_username),
            Err(e) => error!("Failed to create default admin user: {}", e),
        }
    } else {
        info!("Admin user already exists");
    }
    
    Ok(())
}

/// Internal function to create a user
async fn create_user_internal(
    username: String,
    password: String,
    role: UserRole,
) -> Result<User, Box<dyn std::error::Error>> {
    let hashed_password = hash(password, DEFAULT_COST)?;
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: username.clone(),
        email: None, // Email opcional
        password_hash: hashed_password,
        role,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    let mut users = USERS.write().map_err(|e| e.to_string())?;
    
    // Check if username already exists
    if users.iter().any(|u| u.username == user.username) {
        return Err("Username already exists".into());
    }
    
    users.push(user.clone());
    Ok(user)
}

/// Re-export commonly used items
pub use errors::AuthError;
pub use handlers::{
    authenticate,
    create_user,
    get_users,
    get_me,
    delete_user,
};
pub use middleware::{
    require_auth,
    require_admin,
    cors,
    with_user_info,
    rate_limit,
};
pub use models::{
    User,
    UserRole,
    Claims,
    LoginRequest,
    SignupRequest,
    AuthResponse,
    UserResponse,
};
pub use routes::auth_routes;

#[cfg(test)]
mod tests {
    use super::*;

    use bcrypt::verify;
    use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
    use chrono::{Utc, Duration};
    
    #[test]
    fn test_password_hashing() {
        let password = "test_password";
        let hashed = hash(password, DEFAULT_COST).unwrap();
        assert!(verify(password, &hashed).unwrap());
    }
    
    #[test]
    fn test_jwt_token_creation_and_validation() {
        let now = Utc::now();
        let claims = crate::models::Claims {
            sub: "test_user".to_string(),
            role: "User".to_string(),
            exp: (now + Duration::hours(1)).timestamp(),
            iat: now.timestamp(),
        };
        
        // Create token
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        ).unwrap();
        
        assert!(!token.is_empty());
        
        // Verify token
        let token_data = decode::<crate::models::Claims>(
            &token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        );
        
        assert!(token_data.is_ok());
        let token_claims = token_data.unwrap().claims;
        assert_eq!(token_claims.sub, "test_user");
        assert_eq!(token_claims.role, "User");
    }
    
    #[tokio::test]
    async fn test_user_creation() {
        // Clear existing users
        let mut users = USERS.write().unwrap();
        users.clear();
        drop(users); // Release the write lock
        
        // Test user creation
        let result = create_user_internal(
            "testuser".to_string(),
            "testpass".to_string(),
            UserRole::User,
        ).await;
        
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.role, UserRole::User);
        assert!(bcrypt::verify("testpass", &user.password_hash).unwrap());
        
        // Test duplicate username
        let result = create_user_internal(
            "testuser".to_string(),
            "anotherpass".to_string(),
            UserRole::User,
        ).await;
        
        assert!(result.is_err());
    }
}
