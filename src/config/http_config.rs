use crate::compressor::CompressMethod;
use std::time::Duration;

#[derive(Debug)]
pub struct HttpConfig {
    /// Socket读写超时时间，默认10s。
    read_timeout: Duration,

    ///  TCP连接超时时间，默认10s。
    conn_timeout: Duration,

    ///   请求失败重试，默认1，不建议修改，重试由上层业务层处理。
    max_retry_count: u32,

    /// 是否打印请求日志信息，默认false。
    debug_request: bool,

    /// 数据传输压缩方式，默认lz4压缩，支持lz4， deflate,ztsd压缩。
    compress_type: CompressMethod,

    /// 代理服务器主机地址。
    proxy_uri: Option<String>,

    /// 代理服务器验证的用户名。
    proxy_username: Option<String>,

    /// 代理服务器验证的密码。
    proxy_password: Option<String>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            read_timeout: Duration::from_secs(10),
            conn_timeout: Duration::from_secs(10),
            max_retry_count: 1,
            debug_request: false,
            compress_type: CompressMethod::DEFLATE,
            proxy_uri: None,
            proxy_username: None,
            proxy_password: None,
        }
    }
}
impl HttpConfig {
    pub fn read_timeout(&self) -> Duration {
        self.read_timeout
    }

    pub fn conn_timeout(&self) -> Duration {
        self.conn_timeout
    }

    pub fn max_retry_count(&self) -> u32 {
        self.max_retry_count
    }

    pub fn debug_request(&self) -> bool {
        self.debug_request
    }

    pub fn compress_type(&self) -> &CompressMethod {
        &self.compress_type
    }

    pub fn proxy_uri(&self) -> &Option<String> {
        &self.proxy_uri
    }

    pub fn proxy_username(&self) -> &Option<String> {
        &self.proxy_username
    }

    pub fn proxy_password(&self) -> &Option<String> {
        &self.proxy_password
    }

    pub fn set_read_timeout(mut self, read_timeout: Duration) -> Self {
        self.read_timeout = read_timeout;
        self
    }

    pub fn set_conn_timeout(mut self, conn_timeout: Duration) -> Self {
        self.conn_timeout = conn_timeout;
        self
    }

    pub fn set_max_retry_count(mut self, max_retry_count: u32) -> Self {
        self.max_retry_count = max_retry_count;
        self
    }

    pub fn set_debug_request(mut self, debug_request: bool) -> Self {
        self.debug_request = debug_request;
        self
    }

    pub fn set_compress_type(mut self, compress_type: CompressMethod) -> Self {
        self.compress_type = compress_type;
        self
    }

    pub fn set_proxy_uri(mut self, proxy_uri: Option<String>) -> Self {
        self.proxy_uri = proxy_uri;
        self
    }

    pub fn set_proxy_username(mut self, proxy_username: Option<String>) -> Self {
        self.proxy_username = proxy_username;
        self
    }

    pub fn set_proxy_password(mut self, proxy_password: Option<String>) -> Self {
        self.proxy_password = proxy_password;
        self
    }
}
