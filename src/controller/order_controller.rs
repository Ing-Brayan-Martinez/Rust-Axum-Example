use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};

use crate::{
    config::helpers::ServiceError,
    domain::order::Order,
    domain::order::OrderInput,
    service::order_service
};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/order", get(find_all).post(create))
        .route(
            "/api/v1/order/{id}",
            get(find_by_id).put(update).delete(delete),
        )
}

async fn create(Json(input): Json<OrderInput>) -> Result<(StatusCode, Json<Order>), StatusCode> {
    order_service::create(&input.name, input.amount, input.discount)
        .await
        .map(|order| (StatusCode::CREATED, Json(order)))
        .map_err(service_error)
}

async fn update(
    Path(id): Path<i64>,
    Json(input): Json<OrderInput>,
) -> Result<Json<Order>, StatusCode> {
    order_service::update(id, &input.name, input.amount, input.discount)
        .await
        .map(Json)
        .map_err(service_error)
}

async fn delete(Path(id): Path<i64>) -> Result<StatusCode, StatusCode> {
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

async fn find_by_id(Path(id): Path<i64>) -> Result<Json<Order>, StatusCode> {
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
            eprintln!("Order service operation failed: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
