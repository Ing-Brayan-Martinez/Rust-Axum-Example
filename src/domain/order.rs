use std::format;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct OrderCreate {
    pub name: String,
    pub amount: Decimal,
    pub discount: Decimal,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct OrderUpdate {
    pub order_id: Uuid,
    pub name: String,
    pub amount: Decimal,
    pub discount: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Order {
    pub order_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub amount: Decimal,
    pub discount: Decimal,
}

impl Order {
    pub fn to_string(self) -> String {
        format!(
            "Order[ order_id: {}, name: {}, amount: {}, discount: {}, created_at: {}, updated_at: {} ];",
            self.order_id, self.name, self.amount, self.discount, self.created_at, self.updated_at
        )
    }
}
