use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod single_use;
pub use single_use::SingleUseClient;
pub struct TokensClient {
    pub http_client: HttpClient,
    pub single_use: SingleUseClient,
}

impl TokensClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            single_use: SingleUseClient::new(config.clone())?,
        })
    }
}
