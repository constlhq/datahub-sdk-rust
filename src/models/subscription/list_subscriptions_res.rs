use crate::models::subscription::GetSubscriptionRes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListSubscriptionRes {
    subscriptions: Vec<GetSubscriptionRes>,
    total_count: i32,
    request_id: Option<String>,
}

impl ListSubscriptionRes {
    pub fn subscriptions(&self) -> &Vec<GetSubscriptionRes> {
        &self.subscriptions
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_subscriptions(&mut self, subscriptions: Vec<GetSubscriptionRes>) {
        self.subscriptions = subscriptions;
    }

    pub fn set_total_count(&mut self, total_count: i32) {
        self.total_count = total_count;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
