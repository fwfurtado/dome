use crate::{Cipher, PlainText, Secret};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSecretRequest {
    name: String,
    plain_text: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

    use tower::ServiceExt;

    fn url() -> url::Url {
        Faker.fake::<url::Url>()
    }

    static ROUTES: LazyLock<Router> = LazyLock::new(|| {
        routes![
            "/" => routes()
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

        assert_eq!(secret.name, name);
        assert!(!secret.encrypted_text.is_empty());
    }
}
