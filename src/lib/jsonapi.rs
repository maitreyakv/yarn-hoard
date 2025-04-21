use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct JsonApiCreate<T> {
    data: JsonApiCreateData<T>,
}

impl<T> JsonApiCreate<T> {
    pub(crate) fn into_inner(self) -> T {
        self.data.attributes
    }
}

#[derive(Debug, Deserialize)]
struct JsonApiCreateData<T> {
    #[serde(rename = "type")]
    _type: String,
    attributes: T,
}

impl<S, T> FromRequest<S> for JsonApiCreate<T>
where
    Json<JsonApiCreate<T>>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = JsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(create) = Json::<Self>::from_request(req, state).await?;
        Ok(create)
    }
}
