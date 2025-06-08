//! API routes for authentication in the R5 Flowlight application.

use warp::{Filter, Rejection, Reply, http::StatusCode};
use serde_json::json;
use log::{info, error};

use crate::{
    models::{Claims, LoginRequest, SignupRequest},
    handlers::{
        authenticate, create_user, get_me, get_users, delete_user
    },
    errors::AuthError,
    middleware::{
        require_auth, require_admin, cors, handle_rejection, AuthRejection,
    },
};

// Import rate limiting function
use crate::rate_limit::rate_limit;

/// Result type for web handlers
type WebResult<T> = std::result::Result<T, Rejection>;

/// Convert AuthError to Rejection
fn into_rejection(error: AuthError) -> Rejection {
    warp::reject::custom(AuthRejection(error))
}

/// Combine all auth routes with CORS and error handling
pub fn auth_routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // Login route
    let login = warp::path!("auth" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|login: LoginRequest| async move {
            handle_login(login).await
                .map_err(|e| into_rejection(e.into()))
        })
        .boxed()
        .with(warp::trace::named("login"));

    // Signup route
    let signup = warp::path!("auth" / "signup")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|signup: SignupRequest| async move {
            handle_signup(signup).await
                .map_err(|e| into_rejection(e.into()))
        })
        .boxed()
        .with(warp::trace::named("signup"));

    // Get current user info
    let get_me = warp::path!("auth" / "me")
        .and(warp::get())
        .and(require_auth())
        .and_then(|claims: Claims| async move {
            handle_get_me(claims).await
                .map_err(|e| into_rejection(e.into()))
        })
        .boxed()
        .with(warp::trace::named("get_me"));

    // Get all users (admin only)
    let get_users = warp::path!("auth" / "users")
        .and(warp::get())
        .and(require_admin())
        .and_then(|claims: Claims| async move {
            handle_get_users(claims).await
                .map_err(|e| into_rejection(e.into()))
        })
        .boxed()
        .with(warp::trace::named("get_users"));

    // Delete user (admin only)
    let delete_user = warp::path!("auth" / "users" / String)
        .and(warp::delete())
        .and(require_admin())
        .and_then(|user_id: String, _claims: Claims| async move {
            handle_delete_user(user_id).await
                .map_err(|e| into_rejection(e.into()))
        })
        .boxed()
        .with(warp::trace::named("delete_user"));

    // Health check route
    let health = warp::path!("health")
        .and(warp::get())
        .and_then(health_check_handler)
        .boxed()
        .with(warp::trace::named("health_check"));

    // Combine all routes with common middleware
    login
        .or(signup)
        .or(get_me)
        .or(get_users)
        .or(delete_user)
        .or(health)
        .recover(handle_rejection)
        .with(cors())
        .with(warp::log("auth"))
        .boxed()
}

/// Handle login request
async fn handle_login(login: LoginRequest) -> WebResult<impl Reply> {
    info!("Login attempt for user: {}", login.username);
    
    // Apply rate limiting
    rate_limit(&format!("login:{}", login.username));
    
    match authenticate(login).await {
        Ok(response) => {
            info!("Login successful for user: {}", response.user.username);
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ))
        },
        Err(e) => Err(into_rejection(e)),
    }
}

/// Handle signup request
async fn handle_signup(signup: SignupRequest) -> WebResult<impl Reply> {
    info!("Signup attempt for user: {}", signup.username);
    
    // Apply rate limiting
    rate_limit(&format!("signup:{}", signup.username));
    
    match create_user(signup).await {
        Ok(response) => {
            info!("Signup successful for user: {}", response.user.username);
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::CREATED,
            ))
        },
        Err(e) => Err(into_rejection(e)),
    }
}

/// Handle get current user info
async fn handle_get_me(claims: Claims) -> WebResult<impl Reply> {
    info!("Getting current user info for user_id: {}", claims.sub);
    
    match get_me(&claims.sub).await {
        Ok(user) => Ok(warp::reply::with_status(
            warp::reply::json(&user),
            StatusCode::OK,
        )),
        Err(e) => Err(into_rejection(e)),
    }
}

/// Handle get all users (admin only)
async fn handle_get_users(claims: crate::models::Claims) -> WebResult<impl Reply> {
    info!("Fetching all users for admin: {}", claims.sub);
    
    match get_users().await {
        Ok(users) => {
            info!("Successfully fetched {} users", users.len());
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({ "users": users })),
                StatusCode::OK,
            ))
        },
        Err(e) => Err(into_rejection(e)),
    }
}

/// Handle delete user (admin only)
async fn handle_delete_user(user_id: String) -> WebResult<impl Reply> {
    info!("Deleting user: {}", user_id);
    
    match delete_user(&user_id).await {
        Ok(_) => {
            info!("User deleted: {}", user_id);
            Ok(warp::reply::with_status(
                warp::reply::json(&json!({ "status": "ok" })),
                StatusCode::NO_CONTENT,
            ))
        },
        Err(e) => {
            error!("Failed to delete user {}: {}", user_id, e);
            Err(into_rejection(e))
        },
    }
}

/// Health check handler
async fn health_check_handler() -> WebResult<impl Reply> {
    health_check()
        .await
        .map(|_| warp::reply::with_status(
            warp::reply::json(&json!({ "status": "ok" })),
            StatusCode::OK,
        ))
        .map_err(|e| into_rejection(e.into()))
}

/// Health check endpoint
async fn health_check() -> std::result::Result<(), AuthError> {
    Ok(())
}
