use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use uuid::Uuid;

use crate::{
    config::helpers::ServiceError,
    domain::product::{Product, ProductCreate, ProductUpdate},
    service::product_service,
};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/product", get(find_all).post(create).put(update))
        .route("/api/v1/product/{id}", get(find_by_id).delete(delete))
}

async fn create(
    Json(input): Json<ProductCreate>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    product_service::create(input)
        .await
        .map(|product| (StatusCode::CREATED, Json(product)))
        .map_err(service_error)
}

async fn update(Json(input): Json<ProductUpdate>) -> Result<Json<Product>, StatusCode> {
    product_service::update(input)
        .await
        .map(Json)
        .map_err(service_error)
}

async fn delete(Path(id): Path<Uuid>) -> Result<StatusCode, StatusCode> {
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

async fn find_by_id(Path(id): Path<Uuid>) -> Result<Json<Product>, StatusCode> {
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
            tracing::error!(error = %error, "Product service operation failed");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
