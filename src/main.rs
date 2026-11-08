mod config;
mod controller;
mod domain;
mod repository;
mod service;

use axum::{Router, routing::get};
use controller::{index_controller::root, order_controller, product_controller, task_controller};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(root))
        .merge(order_controller::router())
        .merge(product_controller::router())
        .merge(task_controller::router());

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_owned())
        .parse::<u16>()?;
    let address = format!("0.0.0.0:{port}");
    
    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("Server listening on http://localhost:{port}");

    axum::serve(listener, app).await?;
    Ok(())
}
