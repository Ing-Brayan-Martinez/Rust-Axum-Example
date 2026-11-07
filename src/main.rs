mod config;
mod controller;
mod domain;
mod repository;
mod service;

use axum::{Router, routing::get};
use controller::{index_controller::root, order_controller, product_controller, task_controller};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .merge(order_controller::router())
        .merge(product_controller::router())
        .merge(task_controller::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Failed run Axum");

    println!("Server running on http:://localhost:3000")
}
