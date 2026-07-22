use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AudioClient3 {
    pub http_client: HttpClient,
}

impl AudioClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve the first 30 seconds of voice sample audio with or without noise removal.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
    /// * `sample_id` - Sample ID to be used
    /// * `remove_background_noise` - If set will remove background noise for voice samples using our audio isolation model. If the samples do not include background noise, it can make the quality worse.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .voices
    ///         .pvc
    ///         .samples
    ///         .audio
    ///         .get(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"VW7YKqPnjY4h39yTbx2L".to_string(),
    ///             &VoicesPvcSamplesAudioGetQueryRequest {
    ///                 remove_background_noise: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        voice_id: &str,
        sample_id: &str,
        request: &VoicesPvcSamplesAudioGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VoiceSamplePreviewResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/voices/pvc/{}/samples/{}/audio", voice_id, sample_id),
                None,
                QueryBuilder::new()
                    .bool(
                        "remove_background_noise",
                        request.remove_background_noise.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }
}
