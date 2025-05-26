#[derive(Debug)]
pub struct ApiClient {
    http_client: reqwest::Client,
    base_url: String,
    protocol: String,
}

impl ApiClient {
    pub fn secure(base_url: &str) -> Self {
        Self {
            http_client: reqwest::Client::default(),
            base_url: base_url.to_string(),
            protocol: "https".to_string(),
        }
    }

    pub fn insecure(base_url: &str) -> Self {
        Self {
            http_client: reqwest::Client::default(),
            base_url: base_url.to_string(),
            protocol: "http".to_string(),
        }
    }

    pub fn login(&mut self) -> Self {
        todo!()
    }

    pub async fn create_user(
        &self,
        email: &str,
        password: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.http_client
            .post(format!("{}:{}/api/v1/users", self.protocol, self.base_url))
            .json(&serde_json::json!({
                "data": {
                    "type": "users",
                    "attributes": {
                        "email": email,
                        "password": password
                    }
                }
            }))
            .send()
            .await?
            .error_for_status()
    }
}
