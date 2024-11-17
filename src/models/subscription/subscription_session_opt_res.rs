use crate::models::subscription::SubscriptionOffset;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscriptionSessionOptRes {
    offsets: Option<HashMap<String, SubscriptionOffset>>,
    request_id: Option<String>,
}

impl SubscriptionSessionOptRes {
    pub fn offsets(&self) -> &Option<HashMap<String, SubscriptionOffset>> {
        &self.offsets
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_offsets(&mut self, offsets: Option<HashMap<String, SubscriptionOffset>>) {
        self.offsets = offsets;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
