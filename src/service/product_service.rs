use chrono::Utc;
use uuid::Uuid;

use crate::{
    config::helpers::ServiceError,
    domain::product::{Product, ProductCreate, ProductUpdate},
    repository::product_repository,
};

pub async fn create(input: ProductCreate) -> Result<Product, ServiceError> {
    let now = Utc::now().naive_utc();
    let product = Product {
        product_id: Uuid::new_v4(),
        name: input.name,
        price: input.price,
        stock: input.stock,
        created_at: now,
        updated_at: now,
    };

    let saved_product = product_repository::create(product)
        .await
        .map_err(ServiceError::Database)?;

    Ok(saved_product)
}

pub async fn update(input: ProductUpdate) -> Result<Product, ServiceError> {
    let mut product = product_repository::find_by_id(input.product_id)
        .await?
        .ok_or(ServiceError::NotFound)?;

    product.name = input.name;
    product.price = input.price;
    product.stock = input.stock;

    product_repository::update(product)
        .await?
        .ok_or(ServiceError::NotFound)
}

pub async fn delete(product_id: Uuid) -> Result<(), ServiceError> {
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

pub async fn find_by_id(product_id: Uuid) -> Result<Product, ServiceError> {
    product_repository::find_by_id(product_id)
        .await?
        .ok_or(ServiceError::NotFound)
}
