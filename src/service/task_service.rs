use log::info;
use uuid::Uuid;

use crate::{
    config::helpers::ServiceError,
    domain::task::{Task, TaskCreate, TaskUpdate},
    repository::task_repository,
};

#[tracing::instrument(skip(input))]
pub async fn create(input: TaskCreate) -> Result<Task, ServiceError> {
    let task = Task {
        task_id: Uuid::new_v4(),
        name: input.name,
        done: input.done,
    };

    let saved_task = task_repository::create(task)
        .await
        .map_err(ServiceError::Database)?;

    info!("{}", saved_task.clone().to_string());

    Ok(saved_task)
}

#[tracing::instrument(skip(input))]
pub async fn update(input: TaskUpdate) -> Result<Task, ServiceError> {
    let mut task = task_repository::find_by_id(input.task_id)
        .await?
        .ok_or(ServiceError::NotFound)?;

    task.name = input.name;
    task.done = input.done;

    task_repository::update(task)
        .await?
        .ok_or(ServiceError::NotFound)
}

#[tracing::instrument]
pub async fn delete(task_id: Uuid) -> Result<(), ServiceError> {
    if task_repository::delete(task_id).await? {
        Ok(())
    } else {
        Err(ServiceError::NotFound)
    }
}

#[tracing::instrument]
pub async fn find_all() -> Result<Vec<Task>, ServiceError> {
    Ok(task_repository::find_all().await?)
}

#[tracing::instrument]
pub async fn find_by_id(task_id: Uuid) -> Result<Task, ServiceError> {
    task_repository::find_by_id(task_id)
        .await?
        .ok_or(ServiceError::NotFound)
}
