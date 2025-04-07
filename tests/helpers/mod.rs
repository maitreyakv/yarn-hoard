mod test_app;

pub use test_app::TestApp;

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

pub(crate) use jsonapi_create;

pub trait AssertResponse {
    fn assert_status(self, status: u16) -> Self;
}

impl AssertResponse for reqwest::Response {
    fn assert_status(self, status: u16) -> Self {
        assert_eq!(
            self.status(),
            reqwest::StatusCode::from_u16(status).unwrap()
        );
        self
    }
}
