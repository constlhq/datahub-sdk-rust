use crate::errors::ErrorCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorInfo {
    error_code: ErrorCode,
    error_message: String,
}

impl Error for ErrorInfo {
    fn description(&self) -> &str {
        &self.error_message
    }
}

impl Display for ErrorInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{{\"ErrorCode\":\"{}\",\"ErrorMessage\":\"{}\"}}",
            &self.error_code, &self.error_message
        )
    }
}

impl ErrorInfo {
    pub fn error_message(&self) -> &str {
        &self.error_message
    }

    pub fn set_error_message(&mut self, error_message: String) {
        self.error_message = error_message;
    }

    pub fn error_code(&self) -> &ErrorCode {
        &self.error_code
    }

    pub fn set_error_code(&mut self, error_code: ErrorCode) {
        self.error_code = error_code;
    }
}
