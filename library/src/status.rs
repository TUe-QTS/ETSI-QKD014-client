#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Status {
    pub source_kme_id: String,
    pub target_kme_id: String,
    pub source_sae_id: String,
    pub target_sae_id: String,
    pub key_size: u32,
    pub stored_key_count: u32,
    pub max_key_count: u32,
    pub max_key_per_request: u32,
    pub max_key_size: u32,
    pub min_key_size: u32,
    pub max_sae_id_count: u32,
}
