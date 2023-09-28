use std::error::Error;

use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL").expect("Missing DB url.");

    let _conn = sqlx::postgres::PgConnection::connect(&db_connection_str).await?;

    let app = Router::new().route("/home", get(home));

    println!("🚀 Server started successfully!");
    axum::Server::bind(&"127.0.0.1:1337".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn home() -> &'static str {
    "Home"
}
