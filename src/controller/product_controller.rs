use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio::sync::RwLock;

use crate::domain::product::Product;

#[derive(Clone, Default)]
struct ProductState {
    store: Arc<RwLock<ProductStore>>,
}

#[derive(Default)]
struct ProductStore {
    products: HashMap<i64, Product>,
    next_id: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProductInput {
    name: String,
    price: Decimal,
    stock: i32,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/product", get(find_all).post(create))
        .route(
            "/api/v1/product/{id}",
            get(find_by_id).put(update).delete(delete),
        )
        .with_state(ProductState::default())
}

async fn create(
    State(state): State<ProductState>,
    Json(input): Json<ProductInput>,
) -> (StatusCode, Json<Product>) {
    let mut store = state.store.write().await;
    store.next_id += 1;
    let now = Utc::now().naive_utc();
    let product = Product {
        product_id: store.next_id,
        name: input.name,
        price: input.price,
        stock: input.stock,
        created_at: now,
        updated_at: now,
    };
    store.products.insert(product.product_id, product.clone());
    (StatusCode::CREATED, Json(product))
}

async fn update(
    State(state): State<ProductState>,
    Path(id): Path<i64>,
    Json(input): Json<ProductInput>,
) -> Result<Json<Product>, StatusCode> {
    let mut store = state.store.write().await;
    let product = store.products.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
    product.name = input.name;
    product.price = input.price;
    product.stock = input.stock;
    product.updated_at = Utc::now().naive_utc();
    Ok(Json(product.clone()))
}

async fn delete(
    State(state): State<ProductState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    state
        .store
        .write()
        .await
        .products
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn find_all(State(state): State<ProductState>) -> Json<Vec<Product>> {
    let store = state.store.read().await;
    let mut products: Vec<_> = store.products.values().cloned().collect();
    products.sort_by_key(|product| product.product_id);
    Json(products)
}

async fn find_by_id(
    State(state): State<ProductState>,
    Path(id): Path<i64>,
) -> Result<Json<Product>, StatusCode> {
    state
        .store
        .read()
        .await
        .products
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
