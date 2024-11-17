use serde::{Deserialize, Serialize};

mod list_shard_res;
mod merge_shard_res;
mod split_shard_res;

pub use list_shard_res::*;
pub use merge_shard_res::*;
pub use split_shard_res::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum ShardState {
    OPENING,
    ACTIVE,
    CLOSED,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShardInfo {
    shard_id: String,
    state: Option<ShardState>,
    begin_hash_key: String,
    end_hash_key: String,
    parent_shard_ids: Option<Vec<String>>,
}
