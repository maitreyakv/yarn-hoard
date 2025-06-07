use secrecy::{ExposeSecret, SecretString};

#[derive(Clone, Debug)]
pub struct Password(SecretString);

impl Password {
    pub fn expose(&self) -> &str {
        self.0.expose_secret()
    }
}

impl TryFrom<String> for Password {
    type Error = PasswordError;

    fn try_from(password: String) -> Result<Self, Self::Error> {
        if password.len().lt(&8) {
            Err(PasswordError::TooShort)
        } else {
            Ok(Self(SecretString::from(password)))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("Password must be at least 8 characters!")]
    TooShort,
}
