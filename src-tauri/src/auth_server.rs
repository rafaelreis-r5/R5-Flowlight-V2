use warp::Filter;
use log::{info, error};
use auth::{init as auth_init, routes::auth_routes};
use std::net::SocketAddr;

pub async fn start_auth_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize auth module
    auth_init();
    
    // Define routes
    let routes = auth_routes()
        .with(warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    
    info!("Starting authentication server on http://{}", addr);
    
    // Start the server
    warp::serve(routes)
        .run(addr)
        .await;
    
    Ok(())
}
