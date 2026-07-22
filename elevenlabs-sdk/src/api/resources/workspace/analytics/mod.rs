use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod requests;
pub use requests::RequestsClient;
pub struct AnalyticsClient2 {
    pub http_client: HttpClient,
    pub requests: RequestsClient,
}

impl AnalyticsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            requests: RequestsClient::new(config.clone())?,
        })
    }
}
