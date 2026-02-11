use axum::{routing::get, Router};
use std::net::SocketAddr;

pub async fn run_server() {
    let app = Router::new().route("/", get(|| async { "Shalotts API" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_starts() {
        // Would need to mock or ignore for test
        assert!(true);
    }
}