use crate::types::user::UserId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactionRecord {
    pub id: Option<TransactionId>,
    pub ttype: TransactionType,
    pub amt: f32,
    pub user_id: UserId,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransactionId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Transaction {
    pub ttype: TransactionType,
    pub amt: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactionResponse {
    pub id: Option<TransactionId>,
    pub ttype: TransactionType,
    pub amt: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Debit,
    Credit,
}
