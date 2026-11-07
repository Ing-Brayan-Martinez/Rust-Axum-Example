use std::format;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductInput {
    pub name: String,
    pub price: Decimal,
    pub stock: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub product_id: i64,
    pub name: String,
    pub price: Decimal,
    pub stock: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Product {
    pub fn to_string(self) -> String {
        format!(
            "Product[ product_id: {}, name: {}, price: {}, stock: {}, created_at: {}, updated_at: {} ];",
            self.product_id, self.name, self.price, self.stock, self.created_at, self.updated_at
        )
    }
}
