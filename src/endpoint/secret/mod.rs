mod create;

use axum::routing::post;
use axum::Router;

use crate::domain::Cipher;
use crate::gateway::secret::SecretGateway;
use crate::usecase::secret::SecretUseCase;
pub use create::*;

pub fn routes() -> Router {
    let use_case = SecretUseCase::new(Cipher::default(), SecretGateway::new());
    Router::new().route("/", post(create)).with_state(use_case)
}
