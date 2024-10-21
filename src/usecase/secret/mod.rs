use crate::gateway::secret::{SecretGateway, SecretGatewayError};

mod create;

use crate::domain::Cipher;
pub use create::*;

#[derive(Debug)]
pub enum UseCaseError {
    Unknown,
    CipherError(String),
}

impl From<SecretGatewayError> for UseCaseError {
    fn from(_: SecretGatewayError) -> Self {
        UseCaseError::Unknown
    }
}

#[derive(Clone, Debug)]
pub struct SecretUseCase {
    cipher: Cipher,
    gateway: SecretGateway,
}

impl SecretUseCase {
    pub fn new(
        cipher: Cipher,
        gateway: SecretGateway,
    ) -> Self {
        SecretUseCase { cipher, gateway }
    }
}
