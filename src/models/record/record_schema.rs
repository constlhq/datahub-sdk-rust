use crate::models::record::field::Field;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordSchema {
    fields: Vec<Field>,
}

impl RecordSchema {
    //TODO
}
