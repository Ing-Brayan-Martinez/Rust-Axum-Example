use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sqlx::{Row, postgres::PgRow};

use crate::{config::postgresql_conmection::get_db, domain::product::Product};

pub async fn create(name: &str, price: Decimal, stock: i32) -> Result<Product, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query(
        "INSERT INTO products (name, price, stock, created_at, updated_at) \
         VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) \
         RETURNING product_id, name, price, stock, created_at, updated_at",
    )
    .bind(name)
    .bind(price)
    .bind(stock)
    .fetch_one(db)
    .await?;

    row_to_product(row)
}

pub async fn update(
    product_id: i64,
    name: &str,
    price: Decimal,
    stock: i32,
) -> Result<Option<Product>, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query(
        "UPDATE products \
         SET name = $1, price = $2, stock = $3, updated_at = CURRENT_TIMESTAMP \
         WHERE product_id = $4 \
         RETURNING product_id, name, price, stock, created_at, updated_at",
    )
    .bind(name)
    .bind(price)
    .bind(stock)
    .bind(product_id)
    .fetch_optional(db)
    .await?;

    row.map(row_to_product).transpose()
}

pub async fn delete(product_id: i64) -> Result<bool, sqlx::Error> {
    let db = get_db().await?;
    let result = sqlx::query("DELETE FROM products WHERE product_id = $1")
        .bind(product_id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn find_all() -> Result<Vec<Product>, sqlx::Error> {
    let db = get_db().await?;
    sqlx::query(
        "SELECT product_id, name, price, stock, created_at, updated_at \
         FROM products ORDER BY product_id",
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(row_to_product)
    .collect()
}

pub async fn find_by_id(product_id: i64) -> Result<Option<Product>, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query(
        "SELECT product_id, name, price, stock, created_at, updated_at \
         FROM products WHERE product_id = $1",
    )
    .bind(product_id)
    .fetch_optional(db)
    .await?;

    row.map(row_to_product).transpose()
}

// Funciones privada

fn row_to_product(row: PgRow) -> Result<Product, sqlx::Error> {
    Ok(Product {
        product_id: row.try_get("product_id")?,
        name: row.try_get("name")?,
        price: row.try_get("price")?,
        stock: row.try_get("stock")?,
        created_at: row.try_get::<NaiveDateTime, _>("created_at")?,
        updated_at: row.try_get::<NaiveDateTime, _>("updated_at")?,
    })
}
