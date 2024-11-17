use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct EmptyResponse {
    request_id: Option<String>,
}

impl EmptyResponse {
    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
