use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

pub mod secret;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

type HttpResult<T> = Result<StatusWithJson<T>, StatusWithJson<ErrorResponse>>;

type StatusWithJson<T> = (StatusCode, Json<T>);
