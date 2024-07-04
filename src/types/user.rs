use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub user_id: UserId,
    pub nbf: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: Option<UserId>,
    pub email: String,
    pub password: String,
    pub balance: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserCreds {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Balance(pub f32);
