use chrono::NaiveDateTime;
use rust_decimal::Decimal;

pub struct Order {
    pub order_id: i64,
    pub name: String,
    pub amount: Decimal,
    pub discount: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
