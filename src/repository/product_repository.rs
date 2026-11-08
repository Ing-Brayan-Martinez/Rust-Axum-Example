use chrono::NaiveDateTime;
use sqlx::{Row, postgres::PgRow};

use crate::{config::postgresql_conmection::get_db, domain::product::Product};

pub async fn create(product: Product) -> Result<Product, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query(
        "INSERT INTO products (product_id, name, price, stock, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6) \
         RETURNING product_id, name, price, stock, created_at, updated_at",
    )
    .bind(product.product_id)
    .bind(&product.name)
    .bind(product.price)
    .bind(product.stock)
    .bind(product.created_at)
    .bind(product.updated_at)
    .fetch_one(db)
    .await?;

    row_to_product(row)
}

pub async fn update(product: Product) -> Result<Option<Product>, sqlx::Error> {
    let db = get_db().await?;
    let row = sqlx::query(
        "UPDATE products \
         SET name = $1, price = $2, stock = $3, updated_at = CURRENT_TIMESTAMP \
         WHERE product_id = $4 \
         RETURNING product_id, name, price, stock, created_at, updated_at",
    )
    .bind(product.name)
    .bind(product.price)
    .bind(product.stock)
    .bind(product.product_id)
    .fetch_optional(db)
    .await?;

    row.map(row_to_product).transpose()
}

pub async fn delete(product_id: uuid::Uuid) -> Result<bool, sqlx::Error> {
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

pub async fn find_by_id(product_id: uuid::Uuid) -> Result<Option<Product>, sqlx::Error> {
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
