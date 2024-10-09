use crate::json::key_id::KeyId;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct KeysByIdsRequest<'a> {
    #[serde(rename(serialize = "key_IDs"))]
    pub key_ids: Vec<KeyId<'a>>,
}
