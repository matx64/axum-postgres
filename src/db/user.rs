use crate::models::user::User;
use sqlx::{Pool, Postgres};
use std::error::Error;

pub async fn create(pool: &Pool<Postgres>, user: User) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO users (email, password, name) VALUES ($1, $2, $3)")
        .bind(user.email)
        .bind(user.password)
        .bind(user.name)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update(pool: &Pool<Postgres>, user: User) -> Result<(), Box<dyn Error>> {
    sqlx::query("UPDATE user SET email = $1, password = $2, name = $3 WHERE id = $4")
        .bind(user.email)
        .bind(user.password)
        .bind(user.name)
        .bind(user.id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn fetch_all(pool: &Pool<Postgres>) -> Result<Vec<User>, Box<dyn Error>> {
    let result = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(result)
}
