use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct WaveformClient {
    pub http_client: HttpClient,
}

impl WaveformClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve the visual waveform of a voice sample.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
    /// * `sample_id` - Sample ID to be used
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        voice_id: &str,
        sample_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<VoiceSampleVisualWaveformResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/voices/pvc/{}/samples/{}/waveform", voice_id, sample_id),
                None,
                None,
                options,
            )
            .await
    }
}
