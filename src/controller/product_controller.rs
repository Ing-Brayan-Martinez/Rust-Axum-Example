use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{domain::product::Product, repository::product_repository};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProductInput {
    name: String,
    price: Decimal,
    stock: i32,
}

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
    product_repository::create(&input.name, input.price, input.stock)
        .await
        .map(|product| (StatusCode::CREATED, Json(product)))
        .map_err(database_error)
}

async fn update(
    Path(id): Path<i64>,
    Json(input): Json<ProductInput>,
) -> Result<Json<Product>, StatusCode> {
    product_repository::update(id, &input.name, input.price, input.stock)
        .await
        .map_err(database_error)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn delete(Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
    product_repository::delete(id)
        .await
        .map_err(database_error)?
        .then_some(StatusCode::NO_CONTENT)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn find_all() -> Result<Json<Vec<Product>>, StatusCode> {
    product_repository::find_all()
        .await
        .map(Json)
        .map_err(database_error)
}

async fn find_by_id(Path(id): Path<i64>) -> Result<Json<Product>, StatusCode> {
    product_repository::find_by_id(id)
        .await
        .map_err(database_error)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

fn database_error(error: sqlx::Error) -> StatusCode {
    eprintln!("Product database operation failed: {error}");
    StatusCode::INTERNAL_SERVER_ERROR
}
