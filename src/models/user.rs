use sqlx::{FromRow, PgPool, Error};
use serde::Serialize;

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub balance: i32,
    pub avatar: String,
    pub social_id: String
}

impl User {
    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Self, Error> {
        let user = sqlx::query_as!(User, "SELECT id, username, email, balance, avatar, social_id FROM users WHERE username = $1", username)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Self, Error> {
        let user = sqlx::query_as!(User, "SELECT id, username, email, balance, avatar, social_id FROM users WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn get_by_social_id(pool: &PgPool, social_id: &str) -> Result<Self, Error> {
        let user = sqlx::query_as!(User, "SELECT id, username, email, balance, avatar, social_id FROM users WHERE social_id = $1", social_id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }

    pub async fn create(
        pool: &PgPool,
        username: &str,
        email: &str,
        avatar: &str,
        social_id: &str,
    ) -> Result<Self, Error> {
        // In this example, balance is defaulted to 0 on creation.
        let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, balance, avatar, social_id)
        VALUES ($1, $2, 25, $3, $4)
        RETURNING id, username, email, balance, avatar, social_id
        "#,
        username,
        email,
        avatar,
        social_id
    )
            .fetch_one(pool)
            .await?;

        Ok(user)
    }
    pub async fn add_balance(&self, pool: &PgPool, amount: i32) -> Result<Self, Error> {
        sqlx::query!("UPDATE users SET balance = balance + $1 WHERE id = $2", amount, self.id)
            .execute(pool)
            .await?;

        let updated_user = sqlx::query_as!(User, "SELECT id, username, email, balance, avatar, social_id FROM users WHERE id = $1", self.id)
            .fetch_one(pool)
            .await?;

        Ok(updated_user)
    }
}