mod config;
mod controller;
mod domain;
mod repository;
mod service;

use axum::{Router, routing::get};
use controller::{index_controller::root, order_controller, product_controller, task_controller};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, filter::LevelFilter, fmt::format::FmtSpan};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_dotenv()?;

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let app = Router::new()
        .route("/", get(root))
        .merge(order_controller::router())
        .merge(product_controller::router())
        .merge(task_controller::router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        path = %request.uri().path()
                    )
                })
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_owned())
        .parse::<u16>()?;
    let address = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&address).await?;

    info!(%address, "Server listening");

    axum::serve(listener, app).await?;
    Ok(())
}

fn load_dotenv() -> Result<(), dotenvy::Error> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}
