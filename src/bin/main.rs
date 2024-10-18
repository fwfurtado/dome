use dome::domain::{Cipher, PlainText, Secret};

fn main() {
    let plain_text = PlainText::new("My special password".to_string());

    println!("Plain text: {}", plain_text);

    let cipher = Cipher::default();

    let url = url::Url::parse("https://example.com").unwrap();

    let secret = Secret::from_plain_text("My secret".to_string(), url, cipher, plain_text).unwrap();

    println!("Name: {:?}", secret);

    println!("Encrypted: {:?}", secret.encrypted_text());

    let decrypted = secret.show().unwrap();

    println!("Decrypted: {}", decrypted);
}
