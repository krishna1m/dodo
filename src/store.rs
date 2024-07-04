use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use handle_errors::Error;

use crate::types::transaction::TransactionResponse;
use crate::types::{user::{UserCreds, UserId, Balance, User}, transaction::{Transaction, TransactionType, TransactionId}};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn add_user(
        &self,
        new_user: UserCreds,
    ) -> Result<bool, Error> {
        match sqlx::query(
            "INSERT INTO users (email, pwd, balance)
                    VALUES ($1, $2, $3)",
        )
        .bind(new_user.email)
        .bind(new_user.password)
        .bind(0.0)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(error) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = error
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message =
                        error.as_database_error().unwrap().message(),
                    constraint = error
                        .as_database_error()
                        .unwrap()
                        .constraint()
                        .unwrap()
                );
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn get_user(
        &self,
        email: String,
    ) -> Result<User, Error> {
        match sqlx::query("SELECT * FROM users 
                                    WHERE email = $1")
            .bind(email)
            .map(|row: PgRow| User {
                id: Some(UserId(row.get("id"))),
                email: row.get("email"),
                password: row.get("pwd"),
                balance: row.get("balance"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(balance) => Ok(balance),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn get_user_balance(
        &self,
        user_id: &UserId,
    ) -> Result<Balance, Error> {
        match sqlx::query("SELECT balance FROM users 
                                    WHERE id = $1")
            .bind(user_id.0)
            .map(|row: PgRow| Balance(
                row.get("balance")
            ))
            .fetch_one(&self.connection)
            .await
        {
            Ok(balance) => Ok(balance),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn update_user_balance(
        &self,
        user_id: &UserId,
        new_balance: f32,
    ) -> Result<Balance, Error> {
        match sqlx::query(
            "UPDATE users SET balance = $1
                    WHERE id = $2
                    RETURNING balance",
        )
        .bind(new_balance)
        .bind(user_id.0)
        .map(|row: PgRow| Balance(
            row.get("balance")
        ))
        .fetch_one(&self.connection)
        .await
        {
            Ok(balance) => Ok(balance),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_debit(
        &self,
        user_id: &UserId,
        amt: f32,
    ) -> Result<Transaction, Error> {
        match sqlx::query(
            "INSERT INTO transactions (ttype, amt, user_id)
                    VALUES ($1, $2, $3)
                    RETURNING id, ttype, amt",
        )
        .bind(TransactionType::Debit)
        .bind(amt)
        .bind(user_id.0)
        .map(|row: PgRow| Transaction {
            ttype: row.get("ttype"),
            amt: row.get("amt"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(transaction) => Ok(transaction),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn add_credit(
        &self,
        user_id: &UserId,
        amt: f32,
    ) -> Result<Transaction, Error> {
        match sqlx::query(
            "INSERT INTO transactions (ttype, amt, user_id)
                    VALUES ($1, $2, $3)
                    RETURNING ttype, amt",
        )
        .bind(TransactionType::Credit)
        .bind(amt)
        .bind(user_id.0)
        .map(|row: PgRow| Transaction {
            ttype: row.get("ttype"),
            amt: row.get("amt"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(transaction) => Ok(transaction),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }
    

    pub async fn get_transactions(
        &self,
        user_id: UserId,
    ) -> Result<Vec<TransactionResponse>, Error> {
        match sqlx::query(
            "SELECT id, ttype, amt FROM transactions
                 WHERE user_id = $1
                 LIMIT 5"
        )
        .bind(user_id.0)
        .map(|row: PgRow| TransactionResponse {
            id: Some(TransactionId(row.get("id"))),
            ttype: row.get("ttype"),
            amt: row.get("amt"),
        })
        .fetch_all(&self.connection)
        .await
        {
            Ok(transaction) => Ok(transaction),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }
}