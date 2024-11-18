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

impl<'a> QueryCursorPayload<'a> {
    pub fn new(cursor_type: CursorType, parameter: i64) -> Self {
        let mut query_cursor_payload = QueryCursorPayload {
            action: "get",
            r#type: cursor_type,
            system_time: None,
            sequence: None,
        };

        if parameter != -1 {
            match query_cursor_payload.r#type {
                CursorType::SYSTEM_TIME => {
                    query_cursor_payload.system_time = Some(parameter);
                }
                _ => {
                    query_cursor_payload.sequence = Some(parameter);
                }
            }
        }
        query_cursor_payload
    }
}
