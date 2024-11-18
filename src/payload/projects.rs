use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateProjectPayload<'a> {
    pub comment: &'a str,
}

impl<'a> CreateProjectPayload<'a> {
    pub fn new(comment: &'a str) -> Self {
        Self { comment }
    }
}
