use crate::{domain::task::Task, repository::task_repository, config::helpers::ServiceError};

pub async fn create(name: &str, done: bool) -> Result<Task, ServiceError> {
    Ok(task_repository::create(name, done).await?)
}

pub async fn update(id: u32, name: &str, done: bool) -> Result<Task, ServiceError> {
    task_repository::update(id, name, done)
        .await?
        .ok_or(ServiceError::NotFound)
}

pub async fn delete(id: u32) -> Result<(), ServiceError> {
    if task_repository::delete(id).await? {
        Ok(())
    } else {
        Err(ServiceError::NotFound)
    }
}

pub async fn find_all() -> Result<Vec<Task>, ServiceError> {
    Ok(task_repository::find_all().await?)
}

pub async fn find_by_id(id: u32) -> Result<Task, ServiceError> {
    task_repository::find_by_id(id)
        .await?
        .ok_or(ServiceError::NotFound)
}

