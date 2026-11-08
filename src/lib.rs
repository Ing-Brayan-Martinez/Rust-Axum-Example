pub mod config;
pub mod controller;
pub mod domain;
pub mod repository;
pub mod service;

use std::sync::Arc;

use axum::{Router, middleware, routing::get};
use config::api_key::require_api_key;
use controller::{index_controller::root, order_controller, product_controller, task_controller};
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::Level;

pub fn create_app(api_key: Arc<str>) -> Router {
    let api_routes = Router::new()
        .merge(order_controller::router())
        .merge(product_controller::router())
        .merge(task_controller::router())
        .route_layer(middleware::from_fn_with_state(api_key, require_api_key));

    Router::new().route("/", get(root)).merge(api_routes).layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &axum::http::Request<_>| {
                tracing::info_span!(
                    "http_request",
                    method = %request.method(),
                    path = %request.uri().path()
                )
            })
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    )
}
