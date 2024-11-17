#[derive(Debug, Clone)]
pub struct AliyunAccount {
    access_id: String,
    access_key: String,
    security_token: Option<String>,
}

impl AliyunAccount {
    pub fn new(id: String, key: String, token: Option<String>) -> Self {
        Self {
            access_id: id,
            access_key: key,
            security_token: token,
        }
    }

    pub fn id(&self) -> &str {
        &self.access_id
    }

    pub fn key(&self) -> &str {
        &self.access_key
    }

    pub fn token(&self) -> &Option<String> {
        &self.security_token
    }
}
