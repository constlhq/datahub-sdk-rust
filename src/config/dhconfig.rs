use crate::config::account::AliyunAccount;
use crate::config::http_config::HttpConfig;

#[derive(Debug)]
pub struct DatahubConfig {
    endpoint: String,
    account: AliyunAccount,
    http_config: HttpConfig,
    enable_protobuf: bool,
}

impl DatahubConfig {
    pub fn new(
        endpoint: String,
        account: AliyunAccount,
        http_config: HttpConfig,
        enable_protobuf: bool,
    ) -> Self {
        Self {
            endpoint,
            account,
            http_config,
            enable_protobuf,
        }
    }
}

impl DatahubConfig {
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn account(&self) -> &AliyunAccount {
        &self.account
    }

    pub fn http_config(&self) -> &HttpConfig {
        &self.http_config
    }

    pub fn enable_protobuf(&self) -> bool {
        self.enable_protobuf
    }

    pub fn set_endpoint(&mut self, endpoint: String) {
        self.endpoint = endpoint;
    }

    pub fn set_account(&mut self, account: AliyunAccount) {
        self.account = account;
    }

    pub fn set_http_config(&mut self, http_config: HttpConfig) {
        self.http_config = http_config;
    }

    pub fn set_enable_protobuf(&mut self, enable_protobuf: bool) {
        self.enable_protobuf = enable_protobuf;
    }
}
