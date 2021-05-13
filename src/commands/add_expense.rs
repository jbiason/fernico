use crate::commands::interface::Command;
use chrono::DateTime;
use chrono::Utc;
use rust_decimal::prelude::*;

pub struct AddExpense {
    value: Decimal,
    tags: Vec<String>,
    date: DateTime<Utc>,
}
