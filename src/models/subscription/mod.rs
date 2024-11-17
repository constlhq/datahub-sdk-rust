use serde::{Deserialize, Serialize};

mod create_subscription_res;
mod get_subscription_res;
mod list_subscriptions_res;
mod subscription_session_opt_res;

pub use create_subscription_res::*;
pub use get_subscription_res::*;
pub use list_subscriptions_res::*;
pub use subscription_session_opt_res::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscriptionOffset {
    timestamp: i64,
    sequence: i64,
    version: i64,
    session_id: String,
    request_id: Option<String>,
}

impl SubscriptionOffset {
    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp;
    }

    pub fn set_sequence(&mut self, sequence: i64) {
        self.sequence = sequence;
    }

    pub fn set_version(&mut self, version: i64) {
        self.version = version;
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = session_id;
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn sequence(&self) -> i64 {
        self.sequence
    }

    pub fn version(&self) -> i64 {
        self.version
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }
}
