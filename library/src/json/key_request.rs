use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct KeyRequest<'a> {
    pub number: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    #[serde(rename(deserialize = "additional_slave_SAE_IDs"))]
    pub additional_target_sae_ids: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_mandatory: Option<&'a [Map<String, Value>]>,
}
