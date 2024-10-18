mod create;

use axum::routing::post;
use axum::Router;

use create::create;

pub fn routes() -> Router {
    Router::new().route("/", post(create))
}
