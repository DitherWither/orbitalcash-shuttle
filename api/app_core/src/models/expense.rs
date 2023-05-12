use rust_decimal::Decimal;

use super::{user::User, tag::Tag};

pub struct Expense {
    pub expense_id: i32,
    pub user: User,
    pub expense_time: chrono::NaiveDateTime,
    pub amount: Decimal,
    pub tags: Vec<Tag>
}