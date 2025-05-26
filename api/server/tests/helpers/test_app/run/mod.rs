use crate::helpers::TestApp;

impl TestApp {
    pub async fn post_v1_json(&self, path: &str, json: serde_json::Value) -> reqwest::Response {
        self.api_client
            .post(format!(
                "http://localhost:{}/api/v1/{}",
                self.app_port, path
            ))
            .json(&json)
            .send()
            .await
            .unwrap()
    }

    pub async fn put_v1(&self, path: &str) -> reqwest::Response {
        self.api_client
            .put(format!(
                "http://localhost:{}/api/v1/{}",
                self.app_port, path
            ))
            .send()
            .await
            .unwrap()
    }
}
