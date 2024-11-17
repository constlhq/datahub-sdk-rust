use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MergeShardResponse {
    shard_id: String,
    begin_hash_key: String,
    end_hash_key: String,
    request_id: Option<String>,
}

impl MergeShardResponse {
    pub fn shard_id(&self) -> &str {
        &self.shard_id
    }

    pub fn begin_hash_key(&self) -> &str {
        &self.begin_hash_key
    }

    pub fn end_hash_key(&self) -> &str {
        &self.end_hash_key
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_shard_id(&mut self, shard_id: String) {
        self.shard_id = shard_id;
    }

    pub fn set_begin_hash_key(&mut self, begin_hash_key: String) {
        self.begin_hash_key = begin_hash_key;
    }

    pub fn set_end_hash_key(&mut self, end_hash_key: String) {
        self.end_hash_key = end_hash_key;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
