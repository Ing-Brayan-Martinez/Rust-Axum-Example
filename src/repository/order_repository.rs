use crate::{config::postgresql_conmection::get_db, domain::order::Order};
use chrono::NaiveDateTime;
use sqlx::{Row, postgres::PgRow};
use uuid::Uuid;

pub async fn create(order: Order) -> Result<Order, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query(
        "INSERT INTO orders (order_id, name, amount, discount, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6) \
         RETURNING order_id, name, amount, discount, created_at, updated_at",
    )
    .bind(order.order_id)
    .bind(&order.name)
    .bind(order.amount)
    .bind(order.discount)
    .bind(order.created_at)
    .bind(order.updated_at)
    .fetch_one(db)
    .await?;

    row_to_order(row)
}

pub async fn update(order: Order) -> Result<Option<Order>, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query(
        "UPDATE orders \
         SET name = $1, amount = $2, discount = $3, updated_at = CURRENT_TIMESTAMP \
         WHERE order_id = $4 \
         RETURNING order_id, name, amount, discount, created_at, updated_at",
    )
    .bind(order.name)
    .bind(order.amount)
    .bind(order.discount)
    .bind(order.order_id)
    .fetch_optional(db)
    .await?;

    row.map(row_to_order).transpose()
}

pub async fn delete(order_id: Uuid) -> Result<bool, sqlx::Error> {
    let db = get_db().await?;
    let result = sqlx::query("DELETE FROM orders WHERE order_id = $1")
        .bind(order_id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn find_all() -> Result<Vec<Order>, sqlx::Error> {
    let db = get_db().await?;
    sqlx::query(
        "SELECT order_id, name, amount, discount, created_at, updated_at \
         FROM orders ORDER BY order_id",
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(row_to_order)
    .collect()
}

pub async fn find_by_id(order_id: Uuid) -> Result<Option<Order>, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query(
        "SELECT order_id, name, amount, discount, created_at, updated_at \
         FROM orders WHERE order_id = $1",
    )
    .bind(order_id)
    .fetch_optional(db)
    .await?;

    row.map(row_to_order).transpose()
}

// Funciones privada

fn row_to_order(row: PgRow) -> Result<Order, sqlx::Error> {
    Ok(Order {
        order_id: row.try_get("order_id")?,
        name: row.try_get("name")?,
        amount: row.try_get("amount")?,
        discount: row.try_get("discount")?,
        created_at: row.try_get::<NaiveDateTime, _>("created_at")?,
        updated_at: row.try_get::<NaiveDateTime, _>("updated_at")?,
    })
}
