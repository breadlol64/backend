use sqlx::{FromRow, PgPool, Error};
use serde::Serialize;

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    id: i32,
    username: String,
    balance: i32,

}

impl User {
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Self, Error> {
        let user = sqlx::query_as!(User, "SELECT id, username, balance FROM users WHERE username = $1", username)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }
    // pub async fn create(pool: &PgPool, username: &str) -> Result<Self, Error> {
    //     Ok(user)
    // }
    pub async fn add_balance(&self, pool: &PgPool, amount: i32) -> Result<Self, Error> {
        sqlx::query!("UPDATE users SET balance = balance + $1 WHERE id = $2", amount, self.id)
            .execute(pool)
            .await?;

        let updated_user = sqlx::query_as!(User, "SELECT id, username, balance FROM users WHERE id = $1", self.id)
            .fetch_one(pool)
            .await?;

        Ok(updated_user)
    }
}