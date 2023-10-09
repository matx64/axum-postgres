pub mod user;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn db_pool() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_str())
        .await?;

    Ok(pool)
}

fn connection_str() -> String {
    let user = std::env::var("DB_USER").expect("Missing DB_USER");
    let password = std::env::var("DB_PASSWORD").expect("Missing DB_PASSWORD");
    let host = std::env::var("DB_HOST").expect("Missing DB_HOST");
    let port = std::env::var("DB_PORT").expect("Missing DB_PORT");
    let name = std::env::var("DB_NAME").expect("Missing DB_NAME");

    format!(
        "postgres://{}:{}@{}:{}/{}?sslmode=disable",
        user, password, host, port, name
    )
}
