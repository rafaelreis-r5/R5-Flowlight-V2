//! Authentication and authorization middleware for the R5 Flowlight application.

use jsonwebtoken::{decode, DecodingKey, Validation};
use log::debug;
use serde_json::json;
use std::convert::Infallible;
use warp::{
    Filter, Rejection, reject,
    http::{header::{HeaderMap, HeaderValue}, StatusCode},
    reject::Reject,
};

use crate::{
    models::{Claims, UserRole},
    errors::{AuthError, Result},
    JWT_SECRET,
};

/// Custom Rejection for auth errors
#[derive(Debug)]
pub struct AuthRejection(pub AuthError);

impl Reject for AuthRejection {}

impl From<AuthError> for AuthRejection {
    fn from(err: AuthError) -> Self {
        AuthRejection(err)
    }
}

/// Extract JWT token from the Authorization header
pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::header::<String>("authorization")
        .or(warp::header::<String>("Authorization"))
        .unify()
        .and_then(|header: String| async move {
            if header.starts_with("Bearer ") {
                Ok(header[7..].trim().to_string())
            } else {
                debug!("Invalid Authorization header format");
                Err(reject::custom(AuthRejection(AuthError::InvalidToken)))
            }
        })
}

/// Validate JWT token and extract claims
pub fn validate_token(token: &str) -> Result<Claims> {
    let token = token.trim();
    
    if token.is_empty() {
        debug!("Empty token provided");
        return Err(AuthError::InvalidToken);
    }
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    ).map_err(|e| {
        debug!("Token validation failed: {}", e);
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        }
    })?;

    // Check if token is expired (should be handled by the validation)
    if token_data.claims.exp < chrono::Utc::now().timestamp() {
        debug!("Token expired");
        return Err(AuthError::TokenExpired);
    }

    Ok(token_data.claims)
}

/// Require authentication middleware
pub fn require_auth() -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    with_auth()
        .and_then(|token: String| async move {
            validate_token(&token)
                .map_err(|e| {
                    debug!("Authentication failed: {}", e);
                    reject::custom(AuthRejection(e))
                })
        })
}

/// Require specific role middleware
pub fn require_role(required_role: UserRole) -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    let required = required_role;
    warp::any()
        .and(require_auth())
        .and_then(move |claims: Claims| {
            let required = required.clone();
            async move {
                let user_role: UserRole = claims.role.parse().unwrap_or(UserRole::User);
                let user_role_clone = user_role.clone();
                
                match (user_role, &required) {
                    (UserRole::Admin, _) => Ok(claims), // Admins can do anything
                    (UserRole::User, UserRole::User) => Ok(claims), // Regular users can access user endpoints
                    _ => {
                        debug!("Insufficient permissions: {:?} required, but user has {:?}", required, user_role_clone);
                        Err(reject::custom(AuthRejection(AuthError::PermissionDenied)))
                    }
                }
            }
        })
        .boxed()
}

/// Require admin role middleware (shortcut for require_role(UserRole::Admin))
pub fn require_admin() -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    require_role(UserRole::Admin)
}

/// Add user info to request headers for downstream services
pub fn with_user_info(claims: Claims) -> impl Filter<Extract = ((),), Error = Infallible> + Clone {
    let user_id = claims.sub.clone();
    let user_role = claims.role.clone();
    
    warp::any()
        .map(move || {
            let mut headers = HeaderMap::new();
            if let Ok(header_value) = HeaderValue::from_str(&user_id) {
                headers.insert("X-User-Id", header_value);
            }
            if let Ok(header_value) = HeaderValue::from_str(&user_role) {
                headers.insert("X-User-Role", header_value);
            }
            headers
        })
        .map(|_| ())
}

/// Rate limiting middleware (placeholder)
pub fn rate_limit(_key: &str) -> impl Filter<Extract = ((),), Error = Rejection> + Clone {
    // In a real app, you would implement actual rate limiting here
    // For now, just pass through
    warp::any().map(|| (())).boxed()
}

/// CORS middleware
pub fn cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "content-type",
            "authorization",
            "accept",
            "x-requested-with",
        ])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .max_age(300) // 5 minutes
}

/// Rejection handler for authentication errors
pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl warp::Reply, std::convert::Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found")
    } else if let Some(AuthRejection(AuthError::InvalidCredentials)) = err.find() {
        (StatusCode::UNAUTHORIZED, "Invalid email or password")
    } else if let Some(AuthRejection(AuthError::Unauthorized)) = err.find() {
        (StatusCode::UNAUTHORIZED, "Unauthorized")
    } else if let Some(AuthRejection(AuthError::PermissionDenied)) = err.find() {
        (StatusCode::FORBIDDEN, "Insufficient permissions")
    } else if err.find::<AuthRejection>().is_some() {
        (StatusCode::UNAUTHORIZED, "Unauthorized")
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "Method not allowed")
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    };

    let json = warp::reply::json(&json!({
        "error": message,
        "code": code.as_u16(),
    }));

    Ok(warp::reply::with_status(json, code))
}
