use std::{env, io, sync::Arc};

use axum::{
    Json,
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use subtle::ConstantTimeEq;

pub fn load_api_key() -> io::Result<Arc<str>> {
    let api_key = env::var("API_KEY")
        .map_err(|_| io::Error::other("API_KEY must be configured and non-empty"))?;

    if api_key.trim().is_empty() {
        return Err(io::Error::other("API_KEY must be configured and non-empty"));
    }

    Ok(Arc::from(api_key))
}

pub async fn require_api_key(
    State(api_key): State<Arc<str>>,
    request: Request,
    next: Next,
) -> Response {
    let authorized = request
        .headers()
        .get("x-api-key")
        .is_some_and(|provided| api_key_matches(provided, &api_key));

    if authorized {
        next.run(request).await
    } else {
        (StatusCode::UNAUTHORIZED, Json("Unauthorized")).into_response()
    }
}

fn api_key_matches(provided: &HeaderValue, expected: &str) -> bool {
    expected.as_bytes().ct_eq(provided.as_bytes()).unwrap_u8() == 1
}
