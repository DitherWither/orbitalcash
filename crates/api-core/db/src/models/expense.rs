use chrono::naive::serde::ts_milliseconds;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::{tag::Tag, user::User};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Expense {
    pub expense_id: i32,
    pub user: User,
    pub amount: Decimal,
    pub description: String,
    #[serde(with = "ts_milliseconds")]
    pub expense_time: chrono::NaiveDateTime,
    pub tags: Vec<Tag>,
}
