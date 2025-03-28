use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JsonApiCreate<T> {
    data: JsonApiCreateData<T>,
}

impl<T> std::ops::Deref for JsonApiCreate<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data.attributes
    }
}

#[derive(Debug, Deserialize)]
struct JsonApiCreateData<T> {
    #[serde(rename = "type")]
    _type: String,
    attributes: T,
}
