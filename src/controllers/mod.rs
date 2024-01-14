pub mod user;

use axum::http::StatusCode;

pub async fn status() -> StatusCode {
    StatusCode::OK
}
