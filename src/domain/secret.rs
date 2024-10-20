use crate::domain::cipher::{Cipher, CipherError};
use crate::domain::{EncryptedText, PlainText};

#[derive(Debug, Clone)]
pub struct Secret {
    name: String,
    url: url::Url,
    cipher: Cipher,
    encrypted_text: EncryptedText,
}

impl Secret {
    pub fn new(
        name: String,
        url: url::Url,
        cipher: Cipher,
        encrypted_text: EncryptedText,
    ) -> Self {
        Secret {
            name,
            url,
            cipher,
            encrypted_text,
        }
    }

    pub fn show(&self) -> Result<PlainText, CipherError> {
        self.cipher.decrypt(&self.encrypted_text)
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn url(&self) -> url::Url {
        self.url.clone()
    }

    pub fn encrypted_text(&self) -> String {
        self.encrypted_text.show()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::domain::PlainText;
    use fake::faker::company::en::CompanyName;
    use fake::{Fake, Faker};
    use quickcheck::{quickcheck, Arbitrary};
    use url::Url;

    impl Arbitrary for PlainText {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let s: String = Arbitrary::arbitrary(g);
            PlainText::new(s)
        }
    }

    quickcheck! {
        fn should_encrypt_a_string(text: PlainText) -> bool {
            let name = CompanyName().fake();
            let url: Url = Faker.fake::<Url>();

            let cipher = Cipher::default();

            let encrypted = cipher.encrypt(&text).unwrap();

            let secret = Secret::new(name, url, cipher, encrypted);

            let decrypted = secret.show().unwrap();

            text == decrypted
        }
    }
}
