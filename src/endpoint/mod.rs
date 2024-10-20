use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

pub mod secret;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

type HttpResult<T> = Result<T, (StatusCode, Json<ErrorResponse>)>;
