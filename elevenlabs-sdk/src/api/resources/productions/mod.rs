use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod orders;
pub use orders::OrdersClient;
pub struct ProductionsClient {
    pub http_client: HttpClient,
    pub orders: OrdersClient,
}

impl ProductionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            orders: OrdersClient::new(config.clone())?,
        })
    }
}
