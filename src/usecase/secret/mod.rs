use crate::gateway::{SecretGateway, SecretGatewayError};

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

#[derive(Debug)]
pub struct UseCase {
    cipher: Box<Cipher>,
    gateway: Box<dyn SecretGateway>,
}

impl UseCase {
    pub fn new(
        cipher: Box<Cipher>,
        gateway: Box<dyn SecretGateway>,
    ) -> Self {
        UseCase { cipher, gateway }
    }
}
