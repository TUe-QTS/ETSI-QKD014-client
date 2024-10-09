use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct StatusResponse {
    #[serde(rename(deserialize = "source_KME_ID"))]
    pub source_kme_id: String,
    #[serde(rename(deserialize = "target_KME_ID"))]
    pub target_kme_id: String,
    #[serde(rename(deserialize = "master_SAE_ID"))]
    pub source_sae_id: String,
    #[serde(rename(deserialize = "slave_SAE_ID"))]
    pub target_sae_id: String,
    pub key_size: u32,
    pub stored_key_count: u32,
    pub max_key_count: u32,
    pub max_key_per_request: u32,
    pub max_key_size: u32,
    pub min_key_size: u32,
    #[serde(rename(deserialize = "max_SAE_ID_count"))]
    pub max_sae_id_count: u32,
}
