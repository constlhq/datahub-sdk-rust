use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetProjectResponse {
    comment: String,
    create_time: i64,
    last_modify_time: i64,
    request_id: Option<String>,
}

impl GetProjectResponse {
    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
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
