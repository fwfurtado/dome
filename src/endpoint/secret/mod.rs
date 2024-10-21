mod create;

use axum::routing::post;
use axum::Router;

use crate::usecase::secret::SecretUseCase;
pub use create::*;

pub fn routes(use_case: &SecretUseCase) -> Router {
    Router::new()
        .route("/", post(create))
        .with_state(use_case.clone())
}
