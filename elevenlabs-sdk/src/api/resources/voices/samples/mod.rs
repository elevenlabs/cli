use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod audio;
pub use audio::AudioClient5;
pub struct SamplesClient3 {
    pub http_client: HttpClient,
    pub audio: AudioClient5,
}

impl SamplesClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            audio: AudioClient5::new(config.clone())?,
        })
    }
}
