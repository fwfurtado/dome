mod create;

use axum::routing::post;
use axum::Router;

pub use create::*;

pub fn routes() -> Router {
    Router::new().route("/", post(create))
}
