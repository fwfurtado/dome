use crate::gateway::SecretID;
use log::{debug, info};

use crate::secret::CreateSecretRequest;
use crate::usecase::secret::{UseCase, UseCaseError};
use crate::PlainText;
use url::Url;

#[derive(Clone, Debug)]
pub struct CreateSecretCommand {
    name: String,
    value: String,
    url: Url,
}

impl From<CreateSecretRequest> for CreateSecretCommand {
    fn from(value: CreateSecretRequest) -> Self {
        CreateSecretCommand {
            name: value.name,
            value: value.plain_text,
            url: Url::parse(&value.url).unwrap(),
        }
    }
}

impl UseCase {
    pub async fn create(
        &self,
        command: &CreateSecretCommand,
    ) -> Result<SecretID, UseCaseError> {
        info!("Creating secret with name: {}", command.name);

        let name = command.name.clone();
        debug!("Creating secret with name: {}", name);

        let url = command.url.clone();
        debug!("Creating secret with url: {}", url);

        let plain_text = PlainText::new(command.value.clone());
        debug!("Creating secret with plain_text: {}", plain_text);

        match self.cipher.create_secret(name, url, plain_text) {
            Ok(secret) => self.gateway.save(&secret).map_err(UseCaseError::from),
            Err(e) => Err(UseCaseError::CipherError(e.to_string())),
        }
    }
}
