use crate::json::key_and_id::KeyAndId;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct KeyContainer {
    pub keys: Vec<KeyAndId>,
}
