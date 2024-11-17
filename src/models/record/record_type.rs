use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum RecordType {
    TUPLE,
    BLOB,
}
