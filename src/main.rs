mod controllers;
mod db;
mod models;

use axum::{routing::get, Router};
use controllers::user::user_routes;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::error::Error;

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let pool = db::db_pool().await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app_state = AppState { pool };

    let app = Router::new()
        .route("/status", get(controllers::status))
        .nest("/users", user_routes(app_state.clone()));

    println!("🚀 Server started successfully!");
    axum::Server::bind(&"127.0.0.1:1337".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
