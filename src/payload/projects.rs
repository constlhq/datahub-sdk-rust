use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateProjectPayload<'a> {
    pub comment: &'a str,
}
