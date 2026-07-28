use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod api_keys;
pub use api_keys::ApiKeysClient2;
pub struct WorkspacesClient {
    pub http_client: HttpClient,
    pub api_keys: ApiKeysClient2,
}

impl WorkspacesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            api_keys: ApiKeysClient2::new(config.clone())?,
        })
    }
}
