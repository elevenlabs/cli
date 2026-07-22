use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AudioClient4 {
    pub http_client: HttpClient,
}

impl AudioClient4 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve the separated audio for a specific speaker.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
    /// * `sample_id` - Sample ID to be used
    /// * `speaker_id` - Speaker ID to be used, you can use GET https://api.elevenlabs.io/v1/voices/{voice_id}/samples/{sample_id}/speakers to list all the available speakers for a sample.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        voice_id: &str,
        sample_id: &str,
        speaker_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SpeakerAudioResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/voices/pvc/{}/samples/{}/speakers/{}/audio",
                    voice_id, sample_id, speaker_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
