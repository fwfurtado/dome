use crate::endpoint::secret::CreateSecretRequest;
use crate::usecase::secret::CreateSecretCommand;

pub struct SecretAdapter {}

impl SecretAdapter {
    pub fn new() -> Self {
        SecretAdapter {}
    }

    pub fn http_to_command(
        &self,
        input: CreateSecretRequest,
    ) -> CreateSecretCommand {
        CreateSecretCommand {
            name: input.name.clone(),
            value: input.value,
            url: input.url,
        }
    }
}
