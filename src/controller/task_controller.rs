use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::domain::task::Task;

#[derive(Clone, Default)]
struct TaskState {
    store: Arc<RwLock<TaskStore>>,
}

#[derive(Default)]
struct TaskStore {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

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
        .with_state(TaskState::default())
}

async fn create(
    State(state): State<TaskState>,
    Json(input): Json<TaskInput>,
) -> (StatusCode, Json<Task>) {
    let mut store = state.store.write().await;
    store.next_id += 1;
    let task = Task {
        id: store.next_id,
        name: input.name,
        done: input.done,
    };
    store.tasks.insert(task.id, task.clone());
    (StatusCode::CREATED, Json(task))
}

async fn update(
    State(state): State<TaskState>,
    Path(id): Path<u32>,
    Json(input): Json<TaskInput>,
) -> Result<Json<Task>, StatusCode> {
    let mut store = state.store.write().await;
    let task = store.tasks.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
    task.name = input.name;
    task.done = input.done;
    Ok(Json(task.clone()))
}

async fn delete(
    State(state): State<TaskState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    state
        .store
        .write()
        .await
        .tasks
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn find_all(State(state): State<TaskState>) -> Json<Vec<Task>> {
    let store = state.store.read().await;
    let mut tasks: Vec<_> = store.tasks.values().cloned().collect();
    tasks.sort_by_key(|task| task.id);
    Json(tasks)
}

async fn find_by_id(
    State(state): State<TaskState>,
    Path(id): Path<u32>,
) -> Result<Json<Task>, StatusCode> {
    state
        .store
        .read()
        .await
        .tasks
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
