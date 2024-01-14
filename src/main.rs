mod config;
mod controllers;
mod db;
mod models;

pub use config::config;
use db::get_db_pool;

use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "axum_postgres=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let state = AppState {
        pool: get_db_pool().await,
    };

    sqlx::migrate!("./migrations")
        .run(&state.pool)
        .await
        .unwrap();

    let app = Router::new()
        .route("/status", get(controllers::status))
        .nest("/users", controllers::user::user_routes(state.clone()))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::debug!("🚀 Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
