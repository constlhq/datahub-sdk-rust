use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSubscriptionRes {
    sub_id: String,
    request_id: Option<String>,
}

impl CreateSubscriptionRes {
    pub fn sub_id(&self) -> &str {
        &self.sub_id
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_sub_id(&mut self, sub_id: String) {
        self.sub_id = sub_id;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
