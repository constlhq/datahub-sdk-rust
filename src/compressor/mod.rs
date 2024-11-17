#[derive(Debug)]
pub enum CompressMethod {
    ZLIB = 2,
    DEFLATE = 3,
}

impl Default for CompressMethod {
    fn default() -> Self {
        Self::DEFLATE
    }
}
