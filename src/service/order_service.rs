use rust_decimal::Decimal;

use crate::{domain::order::Order, repository::order_repository, config::helpers::ServiceError};

pub async fn create(name: &str, amount: Decimal, discount: Decimal) -> Result<Order, ServiceError> {
    Ok(order_repository::create(name, amount, discount).await?)
}

pub async fn update(
    order_id: i64,
    name: &str,
    amount: Decimal,
    discount: Decimal,
) -> Result<Order, ServiceError> {
    order_repository::update(order_id, name, amount, discount)
        .await?
        .ok_or(ServiceError::NotFound)
}

pub async fn delete(order_id: i64) -> Result<(), ServiceError> {
    if order_repository::delete(order_id).await? {
        Ok(())
    } else {
        Err(ServiceError::NotFound)
    }
}

pub async fn find_all() -> Result<Vec<Order>, ServiceError> {
    Ok(order_repository::find_all().await?)
}

pub async fn find_by_id(order_id: i64) -> Result<Order, ServiceError> {
    order_repository::find_by_id(order_id)
        .await?
        .ok_or(ServiceError::NotFound)
}
