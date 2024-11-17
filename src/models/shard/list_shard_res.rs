use crate::models::shard::ShardInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListShardResponse {
    shards: Vec<ShardInfo>,
    request_id: Option<String>,
}

impl ListShardResponse {
    pub fn set_shards(&mut self, shards: Vec<ShardInfo>) {
        self.shards = shards;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }

    pub fn shards(&self) -> &Vec<ShardInfo> {
        &self.shards
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }
}
