#[derive(Debug)]
pub struct OffsetValue {
    timestamp: i64,
    sequence: i64,
    batch_index: u32,
}

impl OffsetValue {
    pub fn new(timestamp: i64, sequence: i64, batch_index: u32) -> Self {
        Self {
            timestamp,
            sequence,
            batch_index,
        }
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
    pub fn sequence(&self) -> i64 {
        self.sequence
    }
    pub fn batch_index(&self) -> u32 {
        self.batch_index
    }

    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp;
    }
    pub fn set_sequence(&mut self, sequence: i64) {
        self.sequence = sequence;
    }
    pub fn set_batch_index(&mut self, batch_index: u32) {
        self.batch_index = batch_index;
    }
}

impl Default for OffsetValue {
    fn default() -> Self {
        Self {
            timestamp: 1,
            sequence: -1,
            batch_index: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct SubscriptionOffset {
    offset_value: OffsetValue,
    version: i64,
    session_id: i64,
}

impl SubscriptionOffset {
    pub fn new(
        timestamp: i64,
        sequence: i64,
        batch_index: u32,
        version: i64,
        session_id: i64,
    ) -> Self {
        Self {
            offset_value: OffsetValue::new(timestamp, sequence, batch_index),
            version,
            session_id,
        }
    }

    pub fn offset_value(&self) -> &OffsetValue {
        &self.offset_value
    }
    pub fn version(&self) -> i64 {
        self.version
    }
    pub fn session_id(&self) -> i64 {
        self.session_id
    }

    pub fn set_offset_value(&mut self, offset_value: OffsetValue) {
        self.offset_value = offset_value;
    }
    pub fn set_version(&mut self, version: i64) {
        self.version = version;
    }
    pub fn set_session_id(&mut self, session_id: i64) {
        self.session_id = session_id;
    }
}
