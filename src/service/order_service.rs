use crate::domain::order::{OrderCreate, OrderUpdate};
use crate::{config::helpers::ServiceError, domain::order::Order, repository::order_repository};
use chrono::Utc;
use rx_rust::{
    observable::ObservableExt, operators::creating::from_future::FromFuture,
    operators::creating::just::Just, scheduler::runtime::tokio::TokioScheduler,
};
use uuid::Uuid;
use log::info;

#[tracing::instrument(skip(input))]
pub async fn create(input: OrderCreate) -> Result<Order, ServiceError> {
    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let order = Order {
        order_id: id,
        created_at: now,
        updated_at: now,
        name: input.name,
        amount: input.amount,
        discount: input.discount,
    };

    // Llamamos directamente al repositorio de forma asíncrona y manejamos el error con '?'
    let saved_order = order_repository::create(order)
        .await
        .map_err(ServiceError::Database)?;

    info!("{}", saved_order.clone().to_string());
    
    Ok(saved_order)
}

#[tracing::instrument(skip(input))]
pub async fn create2(input: OrderCreate) -> Result<Order, ServiceError> {
    Just::new(input)
        .map(|oi: OrderCreate| {
            let id = Uuid::new_v4();
            let now: chrono::NaiveDateTime = Utc::now().naive_utc();
            let order = Order {
                order_id: id,
                created_at: now,
                updated_at: now,
                name: oi.name,
                amount: oi.amount,
                discount: oi.discount,
            };
            order
        })
        .flat_map(|order: Order| {
            let future = order_repository::create(order);
            FromFuture::new(future, TokioScheduler::current())
        })
        .into_future()
        .await
        .unwrap()
        .map_err(|_| ServiceError::NotFound)
}

#[tracing::instrument(skip(input))]
pub async fn update(input: OrderUpdate) -> Result<Order, ServiceError> {
    let mut order = order_repository::find_by_id(input.order_id).await?.unwrap();

    order.name = input.name;
    order.amount = input.amount;
    order.discount = input.discount;

    order_repository::update(order)
        .await?
        .ok_or(ServiceError::NotFound)
}

#[tracing::instrument]
pub async fn delete(order_id: Uuid) -> Result<(), ServiceError> {
    if order_repository::delete(order_id).await? {
        Ok(())
    } else {
        Err(ServiceError::NotFound)
    }
}

#[tracing::instrument]
pub async fn find_all() -> Result<Vec<Order>, ServiceError> {
    Ok(order_repository::find_all().await?)
}

#[tracing::instrument]
pub async fn find_by_id(order_id: Uuid) -> Result<Order, ServiceError> {
    order_repository::find_by_id(order_id)
        .await?
        .ok_or(ServiceError::NotFound)
}
