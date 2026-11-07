use axum::{Json, Router, extract::Path, http::StatusCode, routing::get};
use serde::Deserialize;

use crate::{domain::task::Task, repository::task_repository};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TaskInput {
    name: String,
    done: bool,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/task", get(find_all).post(create))
        .route(
            "/api/v1/task/{id}",
            get(find_by_id).put(update).delete(delete),
        )
}

async fn create(Json(input): Json<TaskInput>) -> Result<(StatusCode, Json<Task>), StatusCode> {
    task_repository::create(&input.name, input.done)
        .await
        .map(|task| (StatusCode::CREATED, Json(task)))
        .map_err(database_error)
}

async fn update(
    Path(id): Path<u32>,
    Json(input): Json<TaskInput>,
) -> Result<Json<Task>, StatusCode> {
    task_repository::update(id, &input.name, input.done)
        .await
        .map_err(database_error)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn delete(Path(id): Path<u32>) -> Result<StatusCode, StatusCode> {
    task_repository::delete(id)
        .await
        .map_err(database_error)?
        .then_some(StatusCode::NO_CONTENT)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn find_all() -> Result<Json<Vec<Task>>, StatusCode> {
    task_repository::find_all()
        .await
        .map(Json)
        .map_err(database_error)
}

async fn find_by_id(Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
    task_repository::find_by_id(id)
        .await
        .map_err(database_error)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

fn database_error(error: sqlx::Error) -> StatusCode {
    eprintln!("Task database operation failed: {error}");
    StatusCode::INTERNAL_SERVER_ERROR
}
