use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct KeyAndId {
    #[serde(rename(deserialize = "key_ID"))]
    pub key_id: String,
    pub key: String,
}
