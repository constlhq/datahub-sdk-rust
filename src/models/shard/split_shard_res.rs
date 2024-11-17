use crate::models::shard::ShardInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SplitShardResponse {
    new_shards: Vec<ShardInfo>,
    request_id: Option<String>,
}

impl SplitShardResponse {
    pub fn new_shards(&self) -> &Vec<ShardInfo> {
        &self.new_shards
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_new_shards(&mut self, new_shards: Vec<ShardInfo>) {
        self.new_shards = new_shards;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
