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

use crate::domain::order::Order;

#[derive(Clone, Default)]
struct OrderState {
    store: Arc<RwLock<OrderStore>>,
}

#[derive(Default)]
struct OrderStore {
    orders: HashMap<i64, Order>,
    next_id: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OrderInput {
    name: String,
    amount: Decimal,
    discount: Decimal,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/order", get(find_all).post(create))
        .route(
            "/api/v1/order/{id}",
            get(find_by_id).put(update).delete(delete),
        )
        .with_state(OrderState::default())
}

async fn create(
    State(state): State<OrderState>,
    Json(input): Json<OrderInput>,
) -> (StatusCode, Json<Order>) {
    let mut store = state.store.write().await;
    store.next_id += 1;
    let now = Utc::now().naive_utc();
    let order = Order {
        order_id: store.next_id,
        name: input.name,
        amount: input.amount,
        discount: input.discount,
        created_at: now,
        updated_at: now,
    };
    store.orders.insert(order.order_id, order.clone());
    (StatusCode::CREATED, Json(order))
}

async fn update(
    State(state): State<OrderState>,
    Path(id): Path<i64>,
    Json(input): Json<OrderInput>,
) -> Result<Json<Order>, StatusCode> {
    let mut store = state.store.write().await;
    let order = store.orders.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
    order.name = input.name;
    order.amount = input.amount;
    order.discount = input.discount;
    order.updated_at = Utc::now().naive_utc();
    Ok(Json(order.clone()))
}

async fn delete(
    State(state): State<OrderState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    state
        .store
        .write()
        .await
        .orders
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn find_all(State(state): State<OrderState>) -> Json<Vec<Order>> {
    let store = state.store.read().await;
    let mut orders: Vec<_> = store.orders.values().cloned().collect();
    orders.sort_by_key(|order| order.order_id);
    Json(orders)
}

async fn find_by_id(
    State(state): State<OrderState>,
    Path(id): Path<i64>,
) -> Result<Json<Order>, StatusCode> {
    state
        .store
        .read()
        .await
        .orders
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}
