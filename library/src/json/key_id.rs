use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct KeyId<'a> {
    #[serde(rename(serialize = "key_ID"))]
    pub key_id: &'a str,
}
