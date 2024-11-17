use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataUnit {
    Blob(String),
    Tuple(Vec<String>),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    pub shard_id: String,
    pub attributes: Option<HashMap<String, String>>,
    pub data: DataUnit,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WriteDataPayload<'a> {
    pub action: &'a str,
    pub records: Vec<Record>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReadDataPayload<'a> {
    pub action: &'a str,
    pub cursor: &'a str,
    pub limit: i32,
}
