use crate::domain::Secret;
use implementation::SecretGatewayImpl;
use std::fmt::Debug;

mod implementation;

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

pub trait SecretGateway: 'static + Send + Sync + Debug {
    fn save(
        &self,
        secret: &Secret,
    ) -> Result<SecretID, SecretGatewayError>;
}

pub fn new_secret_gateway() -> impl SecretGateway + 'static {
    SecretGatewayImpl::new()
}
