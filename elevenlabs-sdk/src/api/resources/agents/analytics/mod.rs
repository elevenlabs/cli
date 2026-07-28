use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod live_count;
pub use live_count::LiveCountClient;
pub struct AnalyticsClient {
    pub http_client: HttpClient,
    pub live_count: LiveCountClient,
}

impl AnalyticsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            live_count: LiveCountClient::new(config.clone())?,
        })
    }
}
