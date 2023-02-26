mod config;
mod controllers;
mod page;
mod router;

use config::CONFIG;
use router::router_service;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "blog=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = CONFIG.socket_addr;

    axum::Server::bind(&addr)
        .serve(router_service())
        .await
        .expect("Failed to start server.");
}
