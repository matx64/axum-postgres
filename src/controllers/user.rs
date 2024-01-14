use crate::{
    db,
    models::user::{User, UserCreate},
    AppState,
};

use axum::extract::Path;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use uuid::Uuid;

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(get_users))
        .route("/:id", get(get_user))
        .route("/:id", patch(update_user))
        .route("/:id", delete(delete_user))
        .with_state(state)
}

pub async fn create_user(
    state: State<AppState>,
    Json(payload): Json<UserCreate>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = User::new(payload.email, payload.password, payload.name);

    match db::user::create(&state.pool, &user).await {
        Ok(_) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

async fn get_users(state: State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    match db::user::list(&state.pool).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

async fn get_user(state: State<AppState>, Path(id): Path<Uuid>) -> Result<Json<User>, StatusCode> {
    match db::user::get(&state.pool, &id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

async fn update_user(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UserCreate>,
) -> StatusCode {
    let result = db::user::update(&state.pool, &id, &payload)
        .await
        .map(|result| match result.rows_affected() {
            0 => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        });

    match result {
        Ok(status) => status,
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}

async fn delete_user(state: State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    let result =
        db::user::delete(&state.pool, &id)
            .await
            .map(|result| match result.rows_affected() {
                0 => StatusCode::NOT_FOUND,
                _ => StatusCode::OK,
            });

    match result {
        Ok(status) => status,
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}
