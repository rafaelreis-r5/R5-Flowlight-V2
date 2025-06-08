use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, Duration};
use uuid::Uuid;
use bcrypt;
use validator::Validate;
use crate::errors::{Result, AuthError};

/// User roles for authorization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, strum::Display, strum::EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum UserRole {
    User,
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

/// User data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Login request model
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 8, max = 100))]
    pub password: String,
}

/// Signup request model
#[derive(Debug, Deserialize, Validate)]
pub struct SignupRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 8, max = 100))]
    pub password: String,
    #[validate(email)]
    pub email: Option<String>,
}

/// Token claims for JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub role: String, // user role
    pub exp: i64,     // expiration time
    pub iat: i64,     // issued at
}

impl Claims {
    /// Create new claims for a user
    pub fn new(user_id: &str, role: &UserRole, expiration_hours: i64) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expiration_hours);
        
        Self {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }

    /// Create a new JWT token
    pub fn to_token(&self, secret: &[u8]) -> Result<String> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret)
        ).map_err(|e| {
            log::error!("Failed to create JWT token: {}", e);
            AuthError::TokenCreation
        })
    }

    /// Decode a JWT token
    pub fn from_token(token: &str, secret: &[u8]) -> Result<Self> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret),
            &Validation::default()
        ).map_err(|e| {
            log::error!("Failed to decode JWT token: {}", e);
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                _ => AuthError::InvalidToken,
            }
        })?;

        Ok(token_data.claims)
    }
}

/// Authentication response with access and refresh tokens
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
}

/// User response model (without sensitive data)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub role: String,
    pub created_at: chrono::DateTime<Utc>,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.clone(),
            username: user.username.clone(),
            role: user.role.to_string(),
            created_at: user.created_at,
        }
    }
}

impl User {
    /// Create a new user with hashed password
    pub fn new(username: &str, password: &str, role: UserRole) -> Result<Self> {
        let now = Utc::now();
        
        // Validate username
        if username.len() < 3 || username.len() > 50 {
            return Err(AuthError::BadRequest("Username must be between 3 and 50 characters".to_string()));
        }
        
        // Validate password
        if password.len() < 8 {
            return Err(AuthError::BadRequest("Password must be at least 8 characters".to_string()));
        }
        
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            email: None, // Email é opcional
            password_hash: Self::hash_password(password)?,
            role,
            created_at: now,
            updated_at: now,
        })
    }

    /// Verify if the provided password matches the stored hash
    pub fn verify_password(&self, password: &str) -> Result<bool> {
        bcrypt::verify(password, &self.password_hash)
            .map_err(|e| {
                log::error!("Failed to verify password: {}", e);
                AuthError::HashingError
            })
    }

    /// Hash a password using bcrypt
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = bcrypt::DEFAULT_COST;
        bcrypt::hash(password, salt).map_err(|e| {
            log::error!("Failed to hash password: {}", e);
            AuthError::HashingError
        })
    }

    /// Create a new admin user (for initialization)
    pub fn create_admin() -> Result<Self> {
        let username = "SudoAdmin";
        let password = "R5hub2025flowlight";
        
        log::info!("Creating default admin user: {}", username);
        
        Self::new(
            username,
            password,
            UserRole::Admin
        )
    }
    
    /// Convert to response model (without sensitive data)
    pub fn to_response(&self) -> UserResponse {
        UserResponse::from(self)
    }
}
