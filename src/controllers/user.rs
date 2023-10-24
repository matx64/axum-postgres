use crate::{
    db::user::{create, delete, fetch_all, fetch_one_by_id, update},
    models::user::{User, UserCreate},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    routing::post,
    routing::put,
    Json, Router,
};

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_users))
        .route("/", post(create_user))
        .route("/:id", get(get_user_by_id))
        .route("/:id", put(update_user))
        .route("/:id", axum::routing::delete(delete_user))
        .with_state(state)
}

pub async fn get_all_users(state: State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    match fetch_all(&state.pool).await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_user_by_id(
    state: State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<User>, StatusCode> {
    match fetch_one_by_id(&state.pool, &id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            println!("{:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
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

pub async fn update_user(
    state: State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<UserCreate>,
) -> StatusCode {
    let result =
        update(&state.pool, &id, &payload)
            .await
            .map(|result| match result.rows_affected() {
                0 => StatusCode::NOT_FOUND,
                _ => StatusCode::OK,
            });

    match result {
        Ok(status) => status,
        Err(err) => {
            println!("{:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}

pub async fn delete_user(state: State<AppState>, Path(id): Path<uuid::Uuid>) -> StatusCode {
    let result = delete(&state.pool, &id)
        .await
        .map(|result| match result.rows_affected() {
            0 => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        });

    match result {
        Ok(status) => status,
        Err(err) => {
            println!("{:?}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
}
