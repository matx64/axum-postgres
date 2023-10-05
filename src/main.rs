mod db;
mod models;

use db::Db;

use axum::{routing::get, Router};
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db = Db::new().await?;

    sqlx::migrate!("./migrations").run(&db.pool).await?;

    let app = Router::new().route("/status", get(status));

    println!("🚀 Server started successfully!");
    axum::Server::bind(&"127.0.0.1:1337".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn status() -> &'static str {
    "OK"
}
