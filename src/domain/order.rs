use std::format;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInput {
    pub name: String,
    pub amount: Decimal,
    pub discount: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: i64,
    pub name: String,
    pub amount: Decimal,
    pub discount: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Order {
    pub fn to_string(self) -> String {
        format!(
            "Order[ order_id: {}, name: {}, amount: {}, discount: {}, created_at: {}, updated_at: {} ];",
            self.order_id, self.name, self.amount, self.discount, self.created_at, self.updated_at
        )
    }
}
