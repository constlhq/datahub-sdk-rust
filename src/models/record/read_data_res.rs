use crate::models::record::RecordEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReadDataResponse {
    next_cursor: String,
    records: Vec<RecordEntry>,
    request_id: Option<String>,
}

impl ReadDataResponse {
    pub fn next_cursor(&self) -> &str {
        &self.next_cursor
    }

    pub fn records(&self) -> &Vec<RecordEntry> {
        &self.records
    }

    pub fn set_next_cursor(&mut self, next_cursor: String) {
        self.next_cursor = next_cursor;
    }

    pub fn set_records(&mut self, records: Vec<RecordEntry>) {
        self.records = records;
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
