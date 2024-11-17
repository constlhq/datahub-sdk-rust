use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldType {
    UNKNOWN = -1,
    BIGINT = 0,
    DOUBLE = 1,
    BOOLEAN = 2,
    TIMESTAMP = 3,
    STRING = 4,
    DECIMAL = 5,
    INTEGER = 6,
    FLOAT = 7,
    TINYINT = 8,
    SMALLINT = 9,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    name: String,
    r#type: FieldType,
}
