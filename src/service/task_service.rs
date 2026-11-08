use uuid::Uuid;

use crate::{
    config::helpers::ServiceError,
    domain::task::{Task, TaskCreate, TaskUpdate},
    repository::task_repository,
};

pub async fn create(input: TaskCreate) -> Result<Task, ServiceError> {
    let task = Task {
        id: Uuid::new_v4(),
        name: input.name,
        done: input.done,
    };

    let saved_task = task_repository::create(task)
        .await
        .map_err(ServiceError::Database)?;

    Ok(saved_task)
}

pub async fn update(input: TaskUpdate) -> Result<Task, ServiceError> {
    let mut task = task_repository::find_by_id(input.id)
        .await?
        .ok_or(ServiceError::NotFound)?;

    task.name = input.name;
    task.done = input.done;

    task_repository::update(task)
        .await?
        .ok_or(ServiceError::NotFound)
}

pub async fn delete(id: Uuid) -> Result<(), ServiceError> {
    if task_repository::delete(id).await? {
        Ok(())
    } else {
        Err(ServiceError::NotFound)
    }
}

pub async fn find_all() -> Result<Vec<Task>, ServiceError> {
    Ok(task_repository::find_all().await?)
}

pub async fn find_by_id(id: Uuid) -> Result<Task, ServiceError> {
    task_repository::find_by_id(id)
        .await?
        .ok_or(ServiceError::NotFound)
}
