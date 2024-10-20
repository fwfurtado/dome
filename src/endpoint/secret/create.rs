use crate::endpoint::{ErrorResponse, HttpResult};

use crate::usecase::secret;
use crate::usecase::secret::{SecretUseCase, UseCaseError};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSecretRequest {
    pub name: String,
    pub plain_text: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSecretResponse {
    id: String,
}

pub async fn create(
    State(use_case): State<SecretUseCase>,
    Json(input): Json<CreateSecretRequest>,
) -> HttpResult<CreateSecretResponse> {
    debug!("Creating secret with name: {}", input.name);

    let command = secret::CreateSecretCommand::from(input);

    match use_case.create(&command).await {
        Ok(id) => {
            debug!("Secret created");
            Ok((
                StatusCode::CREATED,
                Json(CreateSecretResponse { id: id.into() }),
            ))
        }
        Err(UseCaseError::CipherError(e)) => {
            debug!("Error creating secret: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: 123,
                    message: e,
                }),
            ))
        }
        Err(UseCaseError::Unknown) => {
            debug!("Error creating secret: unknown");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: 500,
                    message: "Unknown error".to_string(),
                }),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::endpoint::secret::routes;
    use crate::routes;
    use axum::{
        body::Body,
        http,
        http::{Request, StatusCode},
        Router,
    };
    use fake::{
        faker::{company::en::CompanyName, internet::en::Password},
        Fake, Faker,
    };
    use std::sync::LazyLock;

    use http_body_util::BodyExt;

    use crate::domain::Cipher;
    use crate::gateway::secret::SecretGateway;
    use tower::ServiceExt;

    fn url() -> url::Url {
        Faker.fake::<url::Url>()
    }

    static ROUTES: LazyLock<Router> = LazyLock::new(|| {
        let use_case = SecretUseCase::new(Cipher::default(), SecretGateway::new());
        routes![
            "/" => routes(&use_case)
        ]
    });

    #[tokio::test]
    async fn should_create_a_secret() {
        let name: String = CompanyName().fake();

        let request_body = CreateSecretRequest {
            name: name.clone(),
            plain_text: Password(8..12).fake(),
            url: url().to_string(),
        };

        let body_json = serde_json::to_string(&request_body).unwrap();

        let response = ROUTES
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();

        let body = String::from_utf8(body.to_vec()).unwrap();

        let secret: CreateSecretResponse = serde_json::from_str(&body).unwrap();

        assert!(!secret.id.is_empty());
    }
}
