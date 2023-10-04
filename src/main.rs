use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_str())
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new().route("/status", get(status));

    println!("🚀 Server started successfully!");
    axum::Server::bind(&"127.0.0.1:1337".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
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

async fn status() -> &'static str {
    "OK"
}
