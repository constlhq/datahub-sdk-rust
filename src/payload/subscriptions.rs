use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSubscriptionPayload<'a> {
    pub action: &'a str,
    pub comment: &'a str,
}

impl<'a> CreateSubscriptionPayload<'a> {
    pub fn new(comment: &'a str) -> Self {
        Self {
            action: "create",
            comment,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListSubscriptionsPayload<'a> {
    pub action: &'a str,
    pub page_index: u32,
    pub page_size: u32,
}

impl ListSubscriptionsPayload<'_> {
    pub fn new(page_index: u32, page_size: u32) -> Self {
        Self {
            action: "list",
            page_index,
            page_size,
        }
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SetSubscriptionStatePayload {
    pub state: i32,
}

impl SetSubscriptionStatePayload {
    pub fn new(state: i32) -> Self {
        Self { state }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscriptionSessionOptPayload<'a> {
    pub action: &'a str,
    pub shard_ids: &'a [&'a str],
}

impl<'a> SubscriptionSessionOptPayload<'a> {
    pub fn open(shard_ids: &'a [&'a str]) -> Self {
        Self {
            action: "open",
            shard_ids,
        }
    }
    pub fn get(shard_ids: &'a [&'a str]) -> Self {
        Self {
            action: "get",
            shard_ids,
        }
    }
}
