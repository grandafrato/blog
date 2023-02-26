mod config;
mod page;
mod router;

use config::CONFIG;
use router::router_service;

#[tokio::main]
async fn main() {
    let addr = CONFIG.socket_addr;

    axum::Server::bind(&addr)
        .serve(router_service())
        .await
        .expect("Failed to start server.");
}
