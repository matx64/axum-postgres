use crate::{db::user::fetch_all, models::user::User, AppState};
use axum::{extract::State, routing::get, routing::post, Json, Router};

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user))
        .with_state(state)
}

pub async fn get_users(state: State<AppState>) -> Result<Json<Vec<User>>, axum::http::StatusCode> {
    match fetch_all(&state.pool).await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_user() -> &'static str {
    "Users"
}
