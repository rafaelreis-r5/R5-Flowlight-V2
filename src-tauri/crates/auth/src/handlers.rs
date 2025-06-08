//! Authentication request handlers for the R5 Flowlight application.

use log::{info, error, warn};
use chrono::Utc;
use jsonwebtoken::{encode, Header, EncodingKey};
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{User, UserRole, Claims, LoginRequest, SignupRequest, AuthResponse, UserResponse},
    errors::{Result, AuthError},
    JWT_SECRET, TOKEN_DURATION_SECONDS,
};

use bcrypt::{hash, verify, DEFAULT_COST};

use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref USERS: RwLock<Vec<User>> = RwLock::new(Vec::new());
}

/// Authenticate a user and return a JWT token
/// 
/// # Arguments
/// * `login` - Login request containing username and password
/// 
/// # Returns
/// * `Result<AuthResponse>` - Authentication response with tokens and user info
pub async fn authenticate(login: LoginRequest) -> Result<AuthResponse> {
    // Validate input
    login.validate().map_err(|e| {
        AuthError::ValidationError(e.field_errors().into_iter()
            .map(|(field, errors)| {
                let error_msg = errors[0].message.as_deref().unwrap_or("invalid");
                format!("{}: {}", field, error_msg)
            })
            .collect())
    })?;

    info!("Authentication attempt for user: {}", login.username);
    
    let users = USERS.read().map_err(|_| {
        error!("Failed to acquire read lock on users store");
        AuthError::InternalError("Failed to access user store".to_string())
    })?;
    
    // Find user by username
    let user = users.iter()
        .find(|u| u.username == login.username)
        .ok_or_else(|| {
            warn!("Authentication failed: user '{}' not found", login.username);
            AuthError::InvalidCredentials
        })?;
    
    // Verify password using bcrypt
    let is_valid = verify(login.password, &user.password_hash)
        .map_err(|_| {
            error!("Password verification failed for user: {}", login.username);
            AuthError::InvalidCredentials
        })?;
    
    if !is_valid {
        warn!("Authentication failed: invalid password for user '{}'", login.username);
        return Err(AuthError::InvalidCredentials);
    }
    
    // Generate tokens
    let access_token = generate_token(&user.id, &user.role.to_string(), *TOKEN_DURATION_SECONDS)?;
    let refresh_token = generate_refresh_token(&user.id)?;
    
    info!("Authentication successful for user: {}", login.username);
    
    Ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: *TOKEN_DURATION_SECONDS,
        user: UserResponse::from(user),
    })
}

/// Create a new user
/// 
/// # Arguments
/// * `signup` - Signup request containing user details
/// * `is_admin` - Whether the current user is an admin (for role assignment)
/// 
/// # Returns
/// * `Result<UserResponse>` - The created user's information (without sensitive data)
pub async fn create_user(signup: SignupRequest) -> Result<AuthResponse> {
    // Validate input
    signup.validate().map_err(|e| {
        AuthError::ValidationError(e.field_errors().into_iter()
            .map(|(field, errors)| {
                let error_msg = errors[0].message.as_deref().unwrap_or("invalid");
                format!("{}: {}", field, error_msg)
            })
            .collect())
    })?;

    info!("Creating new user: {}", signup.username);

    let hashed_password = hash(signup.password, DEFAULT_COST)
        .map_err(|_| AuthError::HashingError)?;

    let user = User {
        id: Uuid::new_v4().to_string(),
        username: signup.username,
        email: signup.email,
        password_hash: hashed_password,
        role: UserRole::User, // Default role is User
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let user_response = UserResponse::from(&user);

    let mut users = USERS.write().map_err(|_| {
        error!("Failed to acquire write lock on users store");
        AuthError::InternalError("Failed to access user store".to_string())
    })?;

    // Check if username or email already exists
    if users.iter().any(|u| u.username == user.username) {
        return Err(AuthError::UserAlreadyExists("Username already exists".to_string()));
    }

    if users.iter().any(|u| u.email == user.email) {
        return Err(AuthError::UserAlreadyExists("Email already exists".to_string()));
    }

    let _user_response = UserResponse::from(&user);
    users.push(user);

    // Generate tokens
    let access_token = generate_token(&_user_response.id, &UserRole::User.to_string(), *TOKEN_DURATION_SECONDS)?;
    let refresh_token = generate_refresh_token(&_user_response.id)?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: *TOKEN_DURATION_SECONDS,
        user: user_response,
    })
}

/// Generate a new access token
pub fn generate_token(user_id: &str, role: &str, expires_in: i64) -> Result<String> {
    let exp_hours = expires_in / 3600; // Convert seconds to hours for Claims::new
    
    // Parse the role string into UserRole
    let user_role: UserRole = role.parse().map_err(|_| {
        error!("Invalid role: {}", role);
        AuthError::InvalidRole
    })?;
    
    let claims = Claims::new(user_id, &user_role, exp_hours);
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes())
    )
    .map_err(|e| {
        error!("Failed to generate token: {}", e);
        AuthError::TokenCreation
    })
}

/// Generate a new refresh token with a longer expiration
pub fn generate_refresh_token(user_id: &str) -> Result<String> {
    // 7 days expiry (in hours)
    let claims = Claims::new(user_id, &UserRole::User, 7 * 24);
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes())
    )
    .map_err(|e| {
        error!("Failed to generate refresh token: {}", e);
        AuthError::TokenCreation
    })
}

/// Get all users (admin only)
pub async fn get_users() -> Result<Vec<UserResponse>> {
    let users = USERS.read().map_err(|_| {
        error!("Failed to acquire read lock on users store");
        AuthError::InternalError("Failed to access user store".to_string())
    })?;
    
    Ok(users.iter().map(|u| UserResponse::from(u)).collect())
}

/// Get current user information
pub async fn get_me(user_id: &str) -> Result<UserResponse> {
    info!("Fetching user info for: {}", user_id);
    
    let users = USERS.read().map_err(|_| {
        error!("Failed to acquire read lock on users store");
        AuthError::InternalError("Failed to access user store".to_string())
    })?;
    
    users.iter()
        .find(|u| u.id == user_id)
        .map(|u| UserResponse::from(u))
        .ok_or_else(|| {
            warn!("User not found: {}", user_id);
            AuthError::UserNotFound
        })
}

/// Delete a user (admin only)
pub async fn delete_user(user_id: &str) -> Result<()> {
    info!("Deleting user: {}", user_id);
    
    let mut users = USERS.write().map_err(|_| {
        error!("Failed to acquire write lock on users store");
        AuthError::InternalError("Failed to access user store".to_string())
    })?;
    
    let initial_len = users.len();
    users.retain(|u| u.id != user_id);
    
    if users.len() == initial_len {
        warn!("User not found for deletion: {}", user_id);
        return Err(AuthError::UserNotFound);
    }
    
    info!("User deleted: {}", user_id);
    Ok(())
}
