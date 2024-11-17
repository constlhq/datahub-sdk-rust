use crate::payload::data::DataUnit;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod field;
mod read_data_res;
mod record_schema;
mod record_type;
mod write_data_res;

pub use field::*;
pub use read_data_res::*;
pub use record_schema::*;
pub use record_type::*;
pub use write_data_res::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecordEntry {
    system_time: i64,
    cursor: String,
    sequence: i64,
    attributes: Option<HashMap<String, String>>,
    data: Option<DataUnit>,
}
