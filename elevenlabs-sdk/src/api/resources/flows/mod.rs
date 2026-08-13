use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod video;
pub use video::VideoClient;
pub mod image;
pub use image::ImageClient;
pub mod text_to_speech;
pub use text_to_speech::TextToSpeechClient2;
pub struct FlowsClient {
    pub http_client: HttpClient,
    pub video: VideoClient,
    pub image: ImageClient,
    pub text_to_speech: TextToSpeechClient2,
}

impl FlowsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            video: VideoClient::new(config.clone())?,
            image: ImageClient::new(config.clone())?,
            text_to_speech: TextToSpeechClient2::new(config.clone())?,
        })
    }
}
