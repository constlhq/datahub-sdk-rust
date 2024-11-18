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

impl<'a> CreateTopicPayload<'a> {
    pub fn new_blob_topic(shard_count: u32, lifecycle: u32, comment: &'a str) -> Self {
        Self {
            action: "create",
            shard_count,
            lifecycle,
            record_type: RecordType::BLOB,
            record_schema: None,
            comment,
            expand_mode: None,
        }
    }
    pub fn new_tuple_topic(
        shard_count: u32,
        lifecycle: u32,
        record_schema: &'a str,
        comment: &'a str,
    ) -> Self {
        Self {
            action: "create",
            shard_count,
            lifecycle,
            record_type: RecordType::TUPLE,
            record_schema: Some(record_schema),
            comment,
            expand_mode: None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AppendFieldPayload<'a> {
    pub action: &'a str,
    pub field_name: &'a str,
    pub field_type: FieldType,
}

impl<'a> AppendFieldPayload<'a> {
    pub fn new(field_name: &'a str, field_type: FieldType) -> Self {
        Self {
            action: "appendfield",
            field_name,
            field_type,
        }
    }
}
