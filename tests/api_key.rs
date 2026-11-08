use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    middleware,
    routing::get,
};
use rust_axum_example::config::api_key::require_api_key;
use tower::ServiceExt;

async fn request_status(api_key: Option<&str>) -> StatusCode {
    let app = Router::new()
        .route("/api/v1/test", get(|| async { "ok" }))
        .route_layer(middleware::from_fn_with_state(
            Arc::<str>::from("expected-key"),
            require_api_key,
        ));

    let mut request = Request::builder().uri("/api/v1/test");
    if let Some(api_key) = api_key {
        request = request.header("x-api-key", api_key);
    }

    app.oneshot(request.body(Body::empty()).unwrap())
        .await
        .unwrap()
        .status()
}

#[tokio::test]
async fn accepts_the_configured_api_key() {
    assert_eq!(request_status(Some("expected-key")).await, StatusCode::OK);
}

#[tokio::test]
async fn rejects_a_missing_api_key() {
    assert_eq!(request_status(None).await, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn rejects_an_incorrect_api_key() {
    assert_eq!(
        request_status(Some("wrong-key")).await,
        StatusCode::UNAUTHORIZED
    );
}
