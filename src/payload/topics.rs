use crate::models::record::{FieldType, RecordType};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateTopicPayload<'a> {
    pub action: &'a str,
    pub shard_count: u32,
    pub lifecycle: u32,
    pub record_type: RecordType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_schema: Option<&'a str>,
    pub comment: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_mode: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AppendFieldPayload<'a> {
    pub action: &'a str,
    pub field_name: &'a str,
    pub field_type: FieldType,
}
