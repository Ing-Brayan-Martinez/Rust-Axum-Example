use sqlx::{Row, postgres::PgRow};

use crate::{config::postgresql_conmection::get_db, domain::task::Task};

pub async fn create(name: &str, done: bool) -> Result<Task, sqlx::Error> {
    let db = get_db().await?;
    let row =
        sqlx::query("INSERT INTO tasks (name, done) VALUES ($1, $2) RETURNING id, name, done")
            .bind(name)
            .bind(done)
            .fetch_one(db)
            .await?;

    row_to_task(row)
}

pub async fn update(id: u32, name: &str, done: bool) -> Result<Option<Task>, sqlx::Error> {
    let db = get_db().await?;
    let id = database_id(id)?;
    let row =
        sqlx::query("UPDATE tasks SET name = $1, done = $2 WHERE id = $3 RETURNING id, name, done")
            .bind(name)
            .bind(done)
            .bind(id)
            .fetch_optional(db)
            .await?;

    row.map(row_to_task).transpose()
}

pub async fn delete(id: u32) -> Result<bool, sqlx::Error> {
    let db = get_db().await?;
    let id = database_id(id)?;
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn find_all() -> Result<Vec<Task>, sqlx::Error> {
    let db = get_db().await?;
    sqlx::query("SELECT id, name, done FROM tasks ORDER BY id")
        .fetch_all(db)
        .await?
        .into_iter()
        .map(row_to_task)
        .collect()
}

pub async fn find_by_id(id: u32) -> Result<Option<Task>, sqlx::Error> {
    let db = get_db().await?;
    let id = database_id(id)?;
    let row = sqlx::query("SELECT id, name, done FROM tasks WHERE id = $1")
        .bind(id)
        .fetch_optional(db)
        .await?;

    row.map(row_to_task).transpose()
}

// Funciones privada

fn database_id(id: u32) -> Result<i32, sqlx::Error> {
    i32::try_from(id).map_err(|error| sqlx::Error::Encode(Box::new(error)))
}

fn row_to_task(row: PgRow) -> Result<Task, sqlx::Error> {
    let id: i32 = row.try_get("id")?;
    let id = u32::try_from(id).map_err(|error| sqlx::Error::ColumnDecode {
        index: "id".to_owned(),
        source: Box::new(error),
    })?;

    Ok(Task {
        id,
        name: row.try_get("name")?,
        done: row.try_get("done")?,
    })
}
