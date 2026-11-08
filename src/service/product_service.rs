use rust_decimal::Decimal;

use crate::{
    config::helpers::ServiceError, domain::product::Product, repository::product_repository,
};

pub async fn create(name: &str, price: Decimal, stock: i32) -> Result<Product, ServiceError> {
    Ok(product_repository::create(name, price, stock).await?)
}

pub async fn update(
    product_id: i64,
    name: &str,
    price: Decimal,
    stock: i32,
) -> Result<Product, ServiceError> {
    product_repository::update(product_id, name, price, stock)
        .await?
        .ok_or(ServiceError::NotFound)
}

pub async fn delete(product_id: i64) -> Result<(), ServiceError> {
    if product_repository::delete(product_id).await? {
        Ok(())
    } else {
        Err(ServiceError::NotFound)
    }
}

pub async fn find_all() -> Result<Vec<Product>, ServiceError> {
    let list = product_repository::find_all().await?;
    Ok(list)
}

pub async fn find_by_id(product_id: i64) -> Result<Product, ServiceError> {
    product_repository::find_by_id(product_id)
        .await?
        .ok_or(ServiceError::NotFound)
}
