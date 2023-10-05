use super::Db;
use crate::models::user::User;

use std::error::Error;

pub async fn create(db: &Db, user: User) -> Result<(), Box<dyn Error>> {
    sqlx::query("INSERT INTO users (email, password, name) VALUES ($1, $2, $3)")
        .bind(user.email)
        .bind(user.password)
        .bind(user.name)
        .execute(&db.pool)
        .await?;
    Ok(())
}

pub async fn fetch_all(db: &Db) -> Result<Vec<User>, Box<dyn Error>> {
    let result = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&db.pool)
        .await?;
    Ok(result)
}
