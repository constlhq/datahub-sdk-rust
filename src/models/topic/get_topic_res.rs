use serde::{Deserialize, Deserializer, Serialize};

use crate::models::record::RecordSchema;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetTopicResponse {
    shard_count: i32,
    lifecycle: i32,
    record_type: String,
    #[serde(deserialize_with = "deserialize_record_schema")]
    record_schema: Option<RecordSchema>,
    comment: String,
    create_time: i64,
    last_modify_time: i64,
    request_id: Option<String>,
}

impl GetTopicResponse {
    pub fn shard_count(&self) -> i32 {
        self.shard_count
    }

    pub fn lifecycle(&self) -> i32 {
        self.lifecycle
    }

    pub fn record_type(&self) -> &str {
        &self.record_type
    }

    pub fn record_schema(&self) -> &Option<RecordSchema> {
        &self.record_schema
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn create_time(&self) -> i64 {
        self.create_time
    }

    pub fn last_modify_time(&self) -> i64 {
        self.last_modify_time
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_shard_count(&mut self, shard_count: i32) {
        self.shard_count = shard_count;
    }

    pub fn set_lifecycle(&mut self, lifecycle: i32) {
        self.lifecycle = lifecycle;
    }

    pub fn set_record_type(&mut self, record_type: String) {
        self.record_type = record_type;
    }

    pub fn set_record_schema(&mut self, record_schema: Option<RecordSchema>) {
        self.record_schema = record_schema;
    }

    pub fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }

    pub fn set_create_time(&mut self, create_time: i64) {
        self.create_time = create_time;
    }

    pub fn set_last_modify_time(&mut self, last_modify_time: i64) {
        self.last_modify_time = last_modify_time;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}

pub(crate) fn deserialize_record_schema<'de, D>(
    deserializer: D,
) -> Result<Option<RecordSchema>, D::Error>
where
    D: Deserializer<'de>,
{
    let input_str: Option<String> = Option::deserialize(deserializer)?;

    match input_str {
        Some(input) => {
            if input.is_empty() {
                Ok(None)
            } else {
                match serde_json::from_str::<RecordSchema>(&input) {
                    Ok(record_schema) => Ok(Some(record_schema)),
                    Err(e) => Err(serde::de::Error::custom(e)),
                }
            }
        }
        None => Ok(None),
    }
}
