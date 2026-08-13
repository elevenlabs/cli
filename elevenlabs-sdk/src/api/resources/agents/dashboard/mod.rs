use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod settings;
pub use settings::SettingsClient2;
pub struct DashboardClient {
    pub http_client: HttpClient,
    pub settings: SettingsClient2,
}

impl DashboardClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            settings: SettingsClient2::new(config.clone())?,
        })
    }
}
