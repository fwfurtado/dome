use crate::gateway::{SecretGateway, SecretGatewayError, SecretID};
use crate::Secret;
use log::info;

#[derive(Debug)]
pub struct SecretGatewayImpl {}

impl SecretGatewayImpl {
    pub fn new() -> Self {
        SecretGatewayImpl {}
    }
}

impl SecretGateway for SecretGatewayImpl {
    fn save(
        &self,
        secret: &Secret,
    ) -> Result<SecretID, SecretGatewayError> {
        info!("Saving secret: {}", secret.name());

        Err(SecretGatewayError::NotImplementedYet(
            "the save method is not implemented yet",
        ))
    }
}
