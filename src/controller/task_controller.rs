use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use uuid::Uuid;

use crate::{
    config::helpers::ServiceError,
    domain::task::{Task, TaskCreate, TaskUpdate},
    service::task_service,
};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/task", get(find_all).post(create).put(update))
        .route("/api/v1/task/{id}", get(find_by_id).delete(delete))
}

async fn create(Json(input): Json<TaskCreate>) -> Result<(StatusCode, Json<Task>), StatusCode> {
    task_service::create(input)
        .await
        .map(|task| (StatusCode::CREATED, Json(task)))
        .map_err(service_error)
}

async fn update(Json(input): Json<TaskUpdate>) -> Result<Json<Task>, StatusCode> {
    task_service::update(input)
        .await
        .map(Json)
        .map_err(service_error)
}

async fn delete(Path(task_id): Path<Uuid>) -> Result<StatusCode, StatusCode> {
    task_service::delete(task_id)
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

async fn find_by_id(Path(task_id): Path<Uuid>) -> Result<Json<Task>, StatusCode> {
    task_service::find_by_id(task_id)
        .await
        .map(Json)
        .map_err(service_error)
}

// Funciones privada

fn service_error(error: ServiceError) -> StatusCode {
    match error {
        ServiceError::NotFound => StatusCode::NOT_FOUND,
        ServiceError::Database(error) => {
            tracing::error!(error = %error, "Task service operation failed");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
