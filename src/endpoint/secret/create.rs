use crate::{Cipher, PlainText, Secret};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateSecretRequest {
    name: String,
    plain_text: String,
    url: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSecretResponse {
    name: String,
    encrypted_text: String,
}

pub async fn create(Json(input): Json<CreateSecretRequest>) -> impl IntoResponse {
    //TODO: extract this to a use case/service layer
    let plain_text = PlainText::new(input.plain_text);

    let cipher = Cipher::default();

    let url = url::Url::parse(input.url.as_str()).unwrap();

    let secret = Secret::from_plain_text(input.name.clone(), url, cipher, plain_text).unwrap();

    (
        StatusCode::CREATED,
        Json(CreateSecretResponse {
            name: input.name.clone(),
            encrypted_text: secret.encrypted_text(),
        }),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn create_secret_test() {
        // let router = Server::default().routes();
    }
}
