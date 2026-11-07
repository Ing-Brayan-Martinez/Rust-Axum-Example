use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use serde::Deserialize;

use crate::{
    domain::task::Task,
    domain::task::TaskInput,
    service::task_service,
    config::helpers::ServiceError
};


pub fn router() -> Router {
    Router::new()
        .route("/api/v1/task", get(find_all).post(create))
        .route(
            "/api/v1/task/{id}",
            get(find_by_id).put(update).delete(delete),
        )
}

async fn create(Json(input): Json<TaskInput>) -> Result<(StatusCode, Json<Task>), StatusCode> {
    task_service::create(&input.name, input.done)
        .await
        .map(|task| (StatusCode::CREATED, Json(task)))
        .map_err(service_error)
}

async fn update(
    Path(id): Path<u32>,
    Json(input): Json<TaskInput>,
) -> Result<Json<Task>, StatusCode> {
    task_service::update(id, &input.name, input.done)
        .await
        .map(Json)
        .map_err(service_error)
}

async fn delete(Path(id): Path<u32>) -> Result<StatusCode, StatusCode> {
    task_service::delete(id)
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(service_error)
}

async fn find_all() -> Result<Json<Vec<Task>>, StatusCode> {
    task_service::find_all()
        .await
        .map(Json)
        .map_err(service_error)
}

async fn find_by_id(Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
    task_service::find_by_id(id)
        .await
        .map(Json)
        .map_err(service_error)
}

// Funciones privada

fn service_error(error: ServiceError) -> StatusCode {
    match error {
        ServiceError::NotFound => StatusCode::NOT_FOUND,
        ServiceError::Database(error) => {
            eprintln!("Task service operation failed: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
