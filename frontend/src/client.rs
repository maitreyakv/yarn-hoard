use email_address::EmailAddress;
use reqwest::StatusCode;

use crate::Password;

macro_rules! jsonapi_create {
    ($t:literal, $a:tt) => {
        ::serde_json::json!({
            "data": {
                "type": $t,
                "attributes": $a
            }
        })
    };
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    http_client: reqwest::Client,
    base_url: String,
    protocol: String,
}

impl ApiClient {
    pub fn secure(base_url: &str) -> Self {
        Self::new(base_url, "https")
    }

    pub fn insecure(base_url: &str) -> Self {
        Self::new(base_url, "http")
    }

    fn new(base_url: &str, protocol: &str) -> Self {
        Self {
            http_client: reqwest::Client::default(),
            base_url: base_url.to_string(),
            protocol: protocol.to_string(),
        }
    }

    pub fn login(&mut self) -> Self {
        todo!()
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn create_user(
        &self,
        email: &EmailAddress,
        password: &Password,
    ) -> Result<(), ApiClientError> {
        self.http_client
            .post(format!("{}:{}/api/v1/users", self.protocol, self.base_url))
            .json(&jsonapi_create!("users", {"email": email, "password": password.expose()}))
            .send()
            .await
            .map_err(ApiClientError::SendFailure)?
            .error_for_status()
            .map(|_| ())
            .map_err(|e| ApiClientError::ErrorStatusCode(e.status().unwrap()))
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn confirm_user_creation(&self, token: &str) -> Result<(), ApiClientError> {
        self.http_client
            .put(format!(
                "{}:{}/api/v1/users/confirm/{}",
                self.protocol, self.base_url, token
            ))
            .send()
            .await
            .map_err(ApiClientError::SendFailure)?
            .error_for_status()
            .map(|_| ())
            .map_err(|e| ApiClientError::ErrorStatusCode(e.status().unwrap()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiClientError {
    #[error("Failed to send request!")]
    SendFailure(#[source] reqwest::Error),

    #[error("Received error status code {0}!")]
    ErrorStatusCode(StatusCode),
}
