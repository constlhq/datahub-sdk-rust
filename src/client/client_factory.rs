use crate::client::datahub_client::DatahubClientTrait;
use crate::config::DatahubConfig;
use crate::DatahubJsonClient;

pub struct DatahubClientFactory;

impl DatahubClientFactory {
    pub fn new_datahub_client(config: &DatahubConfig) -> impl DatahubClientTrait {
        DatahubJsonClient::new(config)
    }
}
