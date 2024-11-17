use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscriptionState {
    OFFLINE = 0,
    ONLINE = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscriptionType {
    USER = 0,
    SYSTEM = 1,
    TT = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionEntry {
    sub_id: String,
    comment: String,
    is_owner: bool,
    sub_type: SubscriptionType,
    state: SubscriptionState,
    create_time: i64,
    last_modify_time: i64,
}

impl SubscriptionEntry {
    pub fn sub_id(&self) -> &str {
        &self.sub_id
    }
    pub fn comment(&self) -> &str {
        &self.comment
    }
    pub fn is_owner(&self) -> bool {
        self.is_owner
    }
    pub fn sub_type(&self) -> &SubscriptionType {
        &self.sub_type
    }
    pub fn state(&self) -> &SubscriptionState {
        &self.state
    }
    pub fn create_time(&self) -> i64 {
        self.create_time
    }
    pub fn last_modify_time(&self) -> i64 {
        self.last_modify_time
    }

    pub fn set_sub_id(&mut self, sub_id: String) {
        self.sub_id = sub_id;
    }
    pub fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }
    pub fn set_is_owner(&mut self, is_owner: bool) {
        self.is_owner = is_owner;
    }
    pub fn set_sub_type(&mut self, sub_type: SubscriptionType) {
        self.sub_type = sub_type;
    }
    pub fn set_state(&mut self, state: SubscriptionState) {
        self.state = state;
    }
    pub fn set_create_time(&mut self, create_time: i64) {
        self.create_time = create_time;
    }
    pub fn set_last_modify_time(&mut self, last_modify_time: i64) {
        self.last_modify_time = last_modify_time;
    }
}
