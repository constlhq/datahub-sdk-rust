use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FailedRecord {
    index: i32,
    error_code: String,
    error_message: String,
}

impl FailedRecord {
    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn error_code(&self) -> &str {
        &self.error_code
    }

    pub fn error_message(&self) -> &str {
        &self.error_message
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WriteDataResponse {
    failed_record_count: i32,
    failed_records: Option<Vec<FailedRecord>>,
    request_id: Option<String>,
}

impl WriteDataResponse {
    pub fn failed_record_count(&self) -> i32 {
        self.failed_record_count
    }

    pub fn failed_records(&self) -> &Option<Vec<FailedRecord>> {
        &self.failed_records
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_failed_record_count(&mut self, failed_record_count: i32) {
        self.failed_record_count = failed_record_count;
    }

    pub fn set_failed_records(&mut self, failed_records: Option<Vec<FailedRecord>>) {
        self.failed_records = failed_records;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
