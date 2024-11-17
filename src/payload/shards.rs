use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SplitShardPayload<'a> {
    pub action: &'a str,
    pub shard_id: &'a str,
    pub split_key: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MergeShardPayload<'a> {
    pub action: &'a str,
    pub shard_id: &'a str,
    pub adjacent_shard_id: &'a str,
}
