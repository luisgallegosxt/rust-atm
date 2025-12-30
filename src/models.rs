use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Account {
    pub balance: f32,
}
#[derive(Debug)]
pub struct Transaction {
    id: Uuid,
    created_on: DateTime<Utc>,
    ttype: String,
    amount: f32,
}

impl Transaction {
    pub fn new(ttype: String, amount: f32) -> Self {
        Transaction {
            id: Uuid::new_v4(),
            created_on: Utc::now(),
            ttype: ttype,
            amount: amount,
        }
    }
}
