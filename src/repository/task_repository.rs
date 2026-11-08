use sqlx::{Row, postgres::PgRow};

use crate::{config::postgresql_conmection::get_db, domain::task::Task};

pub async fn create(task: Task) -> Result<Task, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query(
        "INSERT INTO tasks (task_id, name, done) VALUES ($1, $2, $3) RETURNING task_id, name, done",
    )
    .bind(task.task_id)
    .bind(task.name)
    .bind(task.done)
    .fetch_one(db)
    .await?;

    row_to_task(row)
}

pub async fn update(task: Task) -> Result<Option<Task>, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query(
        "UPDATE tasks SET name = $1, done = $2 WHERE task_id = $3 RETURNING task_id, name, done",
    )
    .bind(task.name)
    .bind(task.done)
    .bind(task.task_id)
    .fetch_optional(db)
    .await?;

    row.map(row_to_task).transpose()
}

pub async fn delete(task_id: uuid::Uuid) -> Result<bool, sqlx::Error> {
    let db = get_db().await?;
    let result = sqlx::query("DELETE FROM tasks WHERE task_id = $1")
        .bind(task_id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn find_all() -> Result<Vec<Task>, sqlx::Error> {
    let db = get_db().await?;
    sqlx::query("SELECT task_id, name, done FROM tasks ORDER BY task_id")
        .fetch_all(db)
        .await?
        .into_iter()
        .map(row_to_task)
        .collect()
}

pub async fn find_by_id(task_id: uuid::Uuid) -> Result<Option<Task>, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query("SELECT task_id, name, done FROM tasks WHERE task_id = $1")
        .bind(task_id)
        .fetch_optional(db)
        .await?;

    row.map(row_to_task).transpose()
}

fn row_to_task(row: PgRow) -> Result<Task, sqlx::Error> {
    Ok(Task {
        task_id: row.try_get("task_id")?,
        name: row.try_get("name")?,
        done: row.try_get("done")?,
    })
}
