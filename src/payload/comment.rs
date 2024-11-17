use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateCommentPayload<'a> {
    pub comment: &'a str,
}
