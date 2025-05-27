use secrecy::{ExposeSecret, SecretString};

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
        email: &str,
        password: &SecretString,
    ) -> Result<(), ApiClientError> {
        Ok(self
            .http_client
            .post(format!("{}:{}/api/v1/users", self.protocol, self.base_url))
            .json(&jsonapi_create!("users", {"email": email, "password": password.expose_secret()}))
            .send()
            .await?
            .error_for_status()
            .map(|_| {})?)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("The API client had an error!")]
pub enum ApiClientError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
