use crate::{
    db::user::{create, fetch_all},
    models::user::{User, UserCreate},
    AppState,
};
use axum::{extract::State, http::StatusCode, routing::get, routing::post, Json, Router};

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user))
        .with_state(state)
}

pub async fn get_users(
    state: State<AppState>,
) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    match fetch_all(&state.pool).await {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_user(
    state: State<AppState>,
    Json(payload): Json<UserCreate>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = User::new(payload.email, payload.password, payload.name);

    match create(&state.pool, &user).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(user))),
        Err(err) => {
            println!("{:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
