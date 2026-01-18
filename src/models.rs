use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Account {
    pub balance: i64,
}
#[derive(Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub created_on: DateTime<Utc>,
    pub ttype: String,
    pub amount: i64,
}

impl Transaction {
    pub fn new(ttype: String, amount: i64) -> Self {
        Transaction {
            id: Uuid::new_v4(),
            created_on: Utc::now(),
            ttype,
            amount,
        }
    }
}
