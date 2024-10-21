use crate::domain::Secret;

use std::fmt::Debug;

use log::info;

#[derive(Debug)]
pub struct SecretID(String);

impl Into<String> for SecretID {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub enum SecretGatewayError {
    Unknown,
    NotImplementedYet(&'static str),
}

#[derive(Clone, Debug)]
pub struct SecretGateway {}

impl SecretGateway {
    pub fn new() -> Self {
        SecretGateway {}
    }

    pub fn save(
        &self,
        secret: &Secret,
    ) -> Result<SecretID, SecretGatewayError> {
        info!("Saving secret: {}", secret.name());

        Err(SecretGatewayError::NotImplementedYet(
            "the save method is not implemented yet",
        ))
    }
}
