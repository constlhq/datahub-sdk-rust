use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryCursorResponse {
    cursor: String,
    record_time: i64,
    sequence: i64,
    request_id: Option<String>,
}

impl QueryCursorResponse {
    pub fn sequence(&self) -> i64 {
        self.sequence
    }

    pub fn record_time(&self) -> i64 {
        self.record_time
    }

    pub fn cursor(&self) -> &str {
        &self.cursor
    }

    pub fn set_cursor(&mut self, cursor: String) {
        self.cursor = cursor;
    }

    pub fn set_record_time(&mut self, record_time: i64) {
        self.record_time = record_time;
    }

    pub fn set_sequence(&mut self, sequence: i64) {
        self.sequence = sequence;
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
