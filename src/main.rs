
mod domain;
mod controller;
mod config;
mod repository;

use controller::index_controller::root;
use axum::{routing::{get, post}, Router, };

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));

    let listener = 
        tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed run Axum");

    println!("Server running on http:://localhost:3000")
}

