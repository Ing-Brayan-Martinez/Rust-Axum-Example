use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};

use crate::{
    config::helpers::ServiceError,
    domain::product::Product,
    domain::product::ProductInput,
    service::product_service
};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/product", get(find_all).post(create))
        .route(
            "/api/v1/product/{id}",
            get(find_by_id).put(update).delete(delete),
        )
}

async fn create(
    Json(input): Json<ProductInput>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    product_service::create(&input.name, input.price, input.stock)
        .await
        .map(|product| (StatusCode::CREATED, Json(product)))
        .map_err(service_error)
}

async fn update(
    Path(id): Path<i64>,
    Json(input): Json<ProductInput>,
) -> Result<Json<Product>, StatusCode> {
    product_service::update(id, &input.name, input.price, input.stock)
        .await
        .map(Json)
        .map_err(service_error)
}

async fn delete(Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
    product_service::delete(id)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(service_error)
}

async fn find_all() -> Result<Json<Vec<Product>>, StatusCode> {
    product_service::find_all()
        .await
        .map(Json)
        .map_err(service_error)
}

async fn find_by_id(Path(id): Path<i64>) -> Result<Json<Product>, StatusCode> {
    product_service::find_by_id(id)
        .await
        .map(Json)
        .map_err(service_error)
}

// Funciones privada

fn service_error(error: ServiceError) -> StatusCode {
    match error {
        ServiceError::NotFound => StatusCode::NOT_FOUND,
        ServiceError::Database(error) => {
            eprintln!("Product service operation failed: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
