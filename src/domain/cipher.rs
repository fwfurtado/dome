use crate::domain::{EncryptedText, PlainText};
use std::fmt::{Debug, Display, Formatter};

use crate::Secret;
use aes_gcm::aes::Aes256;
use aes_gcm::{
    aead::{consts::U12, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, AesGcm, Key, Nonce,
};
use derivative::Derivative;
use url::Url;

#[derive(Clone, Debug)]
pub enum CipherError {
    EncryptionError,
    DecryptionError,
}

impl Display for CipherError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            CipherError::EncryptionError => {
                write!(
                    f,
                    "[EncryptionError] An error occurred while encrypting the text"
                )
            }
            CipherError::DecryptionError => {
                write!(
                    f,
                    "[DecryptionError] An error occurred while decrypting the text"
                )
            }
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct Cipher {
    #[derivative(Debug = "ignore")]
    cipher: Aes256Gcm,
}

impl Default for Cipher {
    fn default() -> Self {
        let key = Aes256Gcm::generate_key(&mut OsRng);

        Cipher::new(key)
    }
}

impl Cipher {
    pub fn new(underlying: Key<AesGcm<Aes256, U12>>) -> Self {
        Cipher {
            cipher: Aes256Gcm::new(&underlying),
        }
    }

    pub fn encrypt(
        &self,
        plain_text: &PlainText,
    ) -> Result<EncryptedText, CipherError> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        self.cipher
            .encrypt(&nonce, plain_text.as_bytes().as_ref())
            .map_err(|_| CipherError::EncryptionError) //TODO: add error logging
            .map(|encrypted| EncryptedText::new(encrypted, nonce.to_vec()))
    }

    pub fn decrypt(
        &self,
        cipher_text: &EncryptedText,
    ) -> Result<PlainText, CipherError> {
        let nonce_bytes = cipher_text.nonce().clone();

        let nonce = Nonce::from_slice(nonce_bytes.as_slice());

        self.cipher
            .decrypt(&nonce, cipher_text.as_bytes())
            .map_err(|_| CipherError::DecryptionError) //TODO: add error logging
            .map(|decrypted| String::from_utf8(decrypted).unwrap())
            .map(PlainText::new)
    }

    pub fn create_secret(
        &self,
        name: String,
        url: Url,
        plain_text: PlainText,
    ) -> Result<Secret, CipherError> {
        let encrypted_text = self.encrypt(&plain_text)?;

        Ok(Secret::new(name, url, self.clone(), encrypted_text))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use aes_gcm::aead::OsRng;
    use quickcheck::quickcheck;

    quickcheck! {
        fn should_encrypt_a_plain_text_string(text: String) -> bool{
            let plain_text = PlainText::new(text.clone());

            let key = Aes256Gcm::generate_key(&mut OsRng);

            let cipher = Cipher::new(key);

            let encrypted = cipher.encrypt(&plain_text).unwrap();

            let decrypted = cipher.decrypt(&encrypted).unwrap();

            plain_text == decrypted
        }
    }
}
