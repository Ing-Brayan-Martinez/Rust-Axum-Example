use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{domain::order::Order, repository::order_repository};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OrderInput {
    name: String,
    amount: Decimal,
    discount: Decimal,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/order", get(find_all).post(create))
        .route(
            "/api/v1/order/{id}",
            get(find_by_id).put(update).delete(delete),
        )
}

async fn create(Json(input): Json<OrderInput>) -> Result<(StatusCode, Json<Order>), StatusCode> {
    order_repository::create(&input.name, input.amount, input.discount)
        .await
        .map(|order| (StatusCode::CREATED, Json(order)))
        .map_err(database_error)
}

async fn update(
    Path(id): Path<i64>,
    Json(input): Json<OrderInput>,
) -> Result<Json<Order>, StatusCode> {
    order_repository::update(id, &input.name, input.amount, input.discount)
        .await
        .map_err(database_error)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn delete(Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
    order_repository::delete(id)
        .await
        .map_err(database_error)?
        .then_some(StatusCode::NO_CONTENT)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn find_all() -> Result<Json<Vec<Order>>, StatusCode> {
    order_repository::find_all()
        .await
        .map(Json)
        .map_err(database_error)
}

async fn find_by_id(Path(id): Path<i64>) -> Result<Json<Order>, StatusCode> {
    order_repository::find_by_id(id)
        .await
        .map_err(database_error)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

fn database_error(error: sqlx::Error) -> StatusCode {
    eprintln!("Order database operation failed: {error}");
    StatusCode::INTERNAL_SERVER_ERROR
}
