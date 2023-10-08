use axum::http::StatusCode;

pub mod user;

pub async fn status() -> StatusCode {
    StatusCode::OK
}
