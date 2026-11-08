use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rust_axum_example::create_app;
use tower::ServiceExt;

#[tokio::test]
async fn health_endpoint_is_public() {
    let app = create_app("expected-key".into());
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn api_endpoints_require_an_api_key() {
    let app = create_app("expected-key".into());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/task")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
