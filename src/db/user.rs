use crate::models::user::User;
use sqlx::{Pool, Postgres};
use std::error::Error;

pub async fn create(pool: &Pool<Postgres>, user: &User) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO users (id, email, password, name, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.name)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update(pool: &Pool<Postgres>, user: &User) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        "UPDATE user SET email = $1, password = $2, name = $3, updated_at = $4 WHERE id = $5",
    )
    .bind(&user.email)
    .bind(&user.password)
    .bind(&user.name)
    .bind(chrono::Utc::now())
    .bind(&user.id)
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
