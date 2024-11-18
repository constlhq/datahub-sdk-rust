use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SplitShardPayload<'a> {
    pub action: &'a str,
    pub shard_id: &'a str,
    pub split_key: &'a str,
}

impl<'a> SplitShardPayload<'a> {
    pub fn new(shard_id: &'a str, split_key: &'a str) -> Self {
        Self {
            action: "split",
            shard_id,
            split_key,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MergeShardPayload<'a> {
    pub action: &'a str,
    pub shard_id: &'a str,
    pub adjacent_shard_id: &'a str,
}

impl<'a> MergeShardPayload<'a> {
    pub fn new(shard_id: &'a str, adjacent_shard_id: &'a str) -> Self {
        Self {
            action: "merge",
            shard_id,
            adjacent_shard_id,
        }
    }
}
