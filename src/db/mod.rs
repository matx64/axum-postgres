pub mod user;

use crate::config;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub async fn get_db_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config().db_url)
        .await
        .expect("Couldn't connect to the database.")
}
