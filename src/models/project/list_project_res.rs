use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListProjectResponse {
    project_names: Vec<String>,
    #[serde(skip)]
    request_id: Option<String>,
}

impl ListProjectResponse {
    pub fn project_names(&self) -> &Vec<String> {
        &self.project_names
    }

    pub fn set_project_names(&mut self, project_names: Vec<String>) {
        self.project_names = project_names;
    }

    pub fn request_id(&self) -> &Option<String> {
        &self.request_id
    }

    pub fn set_request_id(&mut self, request_id: Option<String>) {
        self.request_id = request_id;
    }
}
