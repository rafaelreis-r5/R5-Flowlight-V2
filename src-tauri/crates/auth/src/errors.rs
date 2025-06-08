use jsonwebtoken::errors::Error as JwtError;
use serde::Serialize;
use std::fmt;
use std::error::Error as StdError;
use std::convert::Infallible;
use warp::{
    Rejection,
    http::StatusCode,
    reply::{Json, WithStatus},
};

/// Wrapper para AuthError que implementa Reject
#[derive(Debug)]
pub struct AuthRejection(pub AuthError);

impl std::fmt::Display for AuthRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Auth error: {}", self.0)
    }
}

impl StdError for AuthRejection {}

// Implement Reject manually for AuthError
impl warp::reject::Reject for AuthRejection {}

/// Custom error type for authentication-related errors
#[derive(Debug, Serialize, Clone)]
#[non_exhaustive]
pub enum AuthError {
    InvalidCredentials,
    TokenCreation,
    InvalidToken,
    TokenExpired,
    UserNotFound,
    UserExists,
    HashingError,
    PermissionDenied,
    InternalError(String),
    ValidationError(Vec<String>),
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    BadRequest(String),
    InvalidRole,
    UserAlreadyExists(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidCredentials => write!(f, "Invalid username or password"),
            AuthError::TokenCreation => write!(f, "Failed to create authentication token"),
            AuthError::InvalidToken => write!(f, "Invalid authentication token"),
            AuthError::TokenExpired => write!(f, "Authentication token expired"),
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::UserExists => write!(f, "User already exists"),
            AuthError::HashingError => write!(f, "Password hashing failed"),
            AuthError::PermissionDenied => write!(f, "Insufficient permissions"),
            AuthError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AuthError::ValidationError(errors) => write!(f, "Validation error: {:?}", errors),
            AuthError::Unauthorized => write!(f, "Unauthorized"),
            AuthError::Forbidden => write!(f, "Forbidden"),
            AuthError::NotFound => write!(f, "Not found"),
            AuthError::Conflict => write!(f, "Conflict"),
            AuthError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AuthError::InvalidRole => write!(f, "Invalid role"),
            AuthError::UserAlreadyExists(msg) => write!(f, "User already exists: {}", msg),
        }
    }
}

impl StdError for AuthError {}

impl From<JwtError> for AuthError {
    fn from(err: JwtError) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        }
    }
}

impl From<bcrypt::BcryptError> for AuthError {
    fn from(_: bcrypt::BcryptError) -> Self {
        AuthError::HashingError
    }
}

impl From<Infallible> for AuthError {
    fn from(_: Infallible) -> Self {
        unreachable!("Infallible error occurred")
    }
}

impl From<serde_json::Error> for AuthError {
    fn from(err: serde_json::Error) -> Self {
        AuthError::InternalError(format!("JSON error: {}", err))
    }
}

impl From<std::io::Error> for AuthError {
    fn from(err: std::io::Error) -> Self {
        AuthError::InternalError(format!("IO error: {}", err))
    }
}

impl From<warp::Rejection> for AuthError {
    fn from(rejection: warp::Rejection) -> Self {
        if let Some(auth_error) = rejection.find::<AuthError>() {
            return auth_error.clone();
        }
        
        // Check for status codes
        if rejection.find::<warp::reject::MethodNotAllowed>().is_some() {
            return AuthError::Forbidden;
        }
        
        if rejection.find::<warp::reject::InvalidHeader>().is_some() {
            return AuthError::BadRequest("Invalid header".to_string());
        }
        
        if rejection.find::<warp::reject::MissingHeader>().is_some() {
            return AuthError::BadRequest("Missing required header".to_string());
        }
        
        if rejection.find::<warp::reject::MissingCookie>().is_some() {
            return AuthError::Unauthorized;
        }
        
        if rejection.find::<warp::reject::InvalidQuery>().is_some() {
            return AuthError::BadRequest("Invalid query parameters".to_string());
        }
        
        if rejection.find::<warp::reject::LengthRequired>().is_some() {
            return AuthError::BadRequest("Content length is required".to_string());
        }
        
        if rejection.find::<warp::reject::PayloadTooLarge>().is_some() {
            return AuthError::BadRequest("Payload too large".to_string());
        }
        
        if rejection.find::<warp::reject::UnsupportedMediaType>().is_some() {
            return AuthError::BadRequest("Unsupported media type".to_string());
        }
        
        // If we can't determine the specific error, return a generic one
        if rejection.is_not_found() {
            return AuthError::NotFound;
        }
        
        AuthError::InternalError("An unknown error occurred".to_string())
    }
}

impl warp::reject::Reject for AuthError {}

/// Result type for authentication operations
pub type Result<T> = std::result::Result<T, AuthError>;

/// Convert AuthError to a Warp reply
pub fn handle_rejection(err: Rejection) -> std::result::Result<WithStatus<Json>, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found")
    } else if let Some(e) = err.find::<AuthError>() {
        match e {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
            AuthError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            AuthError::UserExists => (StatusCode::CONFLICT, "User already exists"),
            AuthError::HashingError => (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing failed"),
            AuthError::PermissionDenied => (StatusCode::FORBIDDEN, "Insufficient permissions"),
            AuthError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AuthError::ValidationError(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden"),
            AuthError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AuthError::Conflict => (StatusCode::CONFLICT, "Conflict"),
            AuthError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AuthError::InvalidRole => (StatusCode::BAD_REQUEST, "Invalid user role"),
            AuthError::UserAlreadyExists(msg) => (StatusCode::CONFLICT, msg.as_str()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "Method not allowed")
    } else {
        eprintln!("Unhandled rejection: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    };

    let json = warp::reply::json(&serde_json::json!({
        "error": message,
        "code": code.as_u16(),
    }));

    Ok(warp::reply::with_status(json, code))
}
