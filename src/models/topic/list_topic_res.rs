use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTopicResponse {
    topic_names: Vec<String>,
    request_id: Option<String>,
}

impl ListTopicResponse {
    pub fn topic_names(&self) -> &Vec<String> {
        &self.topic_names
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_topic_names(&mut self, topic_names: Vec<String>) {
        self.topic_names = topic_names;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
