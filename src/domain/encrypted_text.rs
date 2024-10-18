#[derive(Debug, Clone)]
pub struct EncryptedText {
    text: Vec<u8>,
    nonce: Vec<u8>,
}

impl EncryptedText {
    pub fn new(
        text: Vec<u8>,
        nonce: Vec<u8>,
    ) -> Self {
        EncryptedText { text, nonce }
    }

    pub fn nonce(&self) -> Vec<u8> {
        self.nonce.clone()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.text.as_ref()
    }

    pub fn show(&self) -> String {
        hex::encode(self.text.clone())
    }
}
