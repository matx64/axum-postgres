use crate::models::user::{User, UserCreate};
use sqlx::{postgres::PgQueryResult, Pool, Postgres};

pub async fn fetch_all(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    let result = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn fetch_one_by_id(
    pool: &Pool<Postgres>,
    id: &uuid::Uuid,
) -> Result<Option<User>, sqlx::Error> {
    let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(result)
}

pub async fn create(pool: &Pool<Postgres>, user: &User) -> Result<(), sqlx::Error> {
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

pub async fn update(
    pool: &Pool<Postgres>,
    id: &uuid::Uuid,
    user: &UserCreate,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query(
        "UPDATE users SET email = $1, password = $2, name = $3, updated_at = $4 WHERE id = $5",
    )
    .bind(&user.email)
    .bind(&user.password)
    .bind(&user.name)
    .bind(chrono::Utc::now())
    .bind(&id)
    .execute(pool)
    .await
}

pub async fn delete(pool: &Pool<Postgres>, id: &uuid::Uuid) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(&id)
        .execute(pool)
        .await
}
