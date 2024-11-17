#![allow(non_camel_case_types)]

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum CursorType {
    OLDEST,
    LATEST,
    SYSTEM_TIME,
    SEQUENCE,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryCursorPayload<'a> {
    pub action: &'a str,
    pub r#type: CursorType,
    pub system_time: Option<i64>,
    pub sequence: Option<i64>,
}
