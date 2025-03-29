use secrecy::{ExposeSecret, SecretString};

pub fn generate_salt() -> SecretString {
    let mut salt_bytes = [0; 6].to_vec();
    openssl::rand::rand_bytes(&mut salt_bytes).unwrap();
    SecretString::from(openssl::base64::encode_block(&salt_bytes))
}

pub fn hash_password(password: &SecretString, salt: &SecretString) -> SecretString {
    SecretString::from(hex::encode(openssl::sha::sha256(
        format!("{}{}", password.expose_secret(), salt.expose_secret()).as_bytes(),
    )))
}
