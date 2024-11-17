use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSubscriptionRes {
    sub_id: String,
    comment: String,
    state: i32,
    request_id: Option<String>,
}

impl GetSubscriptionRes {
    pub fn sub_id(&self) -> &str {
        &self.sub_id
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn state(&self) -> i32 {
        self.state
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_sub_id(&mut self, sub_id: String) {
        self.sub_id = sub_id;
    }

    pub fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }

    pub fn set_state(&mut self, state: i32) {
        self.state = state;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
