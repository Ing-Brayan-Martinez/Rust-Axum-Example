use axum::routing::post;
use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use uuid::Uuid;

use crate::{
    config::helpers::ServiceError, domain::order::Order, domain::order::OrderCreate,
    domain::order::OrderUpdate, service::order_service,
};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/order", get(find_all).post(create).put(update))
        .route("/api/v1/order2", post(create2))
        .route("/api/v1/order/{id}", get(find_by_id).delete(delete))
}

async fn create(Json(input): Json<OrderCreate>) -> Result<(StatusCode, Json<Order>), StatusCode> {
    order_service::create(input)
        .await
        .map(|order| (StatusCode::CREATED, Json(order)))
        .map_err(service_error)
}

async fn create2(Json(input): Json<OrderCreate>) -> Result<(StatusCode, Json<Order>), StatusCode> {
    order_service::create2(input)
        .await
        .map(|order| (StatusCode::CREATED, Json(order)))
        .map_err(service_error)
}

async fn update(Json(input): Json<OrderUpdate>) -> Result<(StatusCode, Json<Order>), StatusCode> {
    order_service::update(input)
        .await
        .map(|order| (StatusCode::OK, Json(order)))
        .map_err(service_error)
}

async fn delete(Path(id): Path<Uuid>) -> Result<StatusCode, StatusCode> {
    order_service::delete(id)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(service_error)
}

async fn find_all() -> Result<Json<Vec<Order>>, StatusCode> {
    order_service::find_all()
        .await
        .map(Json)
        .map_err(service_error)
}

async fn find_by_id(Path(id): Path<Uuid>) -> Result<Json<Order>, StatusCode> {
    order_service::find_by_id(id)
        .await
        .map(Json)
        .map_err(service_error)
}

// Funciones privada

fn service_error(error: ServiceError) -> StatusCode {
    match error {
        ServiceError::NotFound => StatusCode::NOT_FOUND,
        ServiceError::Database(error) => {
            tracing::error!(error = %error, "Order service operation failed");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
