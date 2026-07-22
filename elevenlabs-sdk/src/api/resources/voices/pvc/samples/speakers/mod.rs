use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod audio;
pub use audio::AudioClient4;
pub struct SpeakersClient {
    pub http_client: HttpClient,
    pub audio: AudioClient4,
}

impl SpeakersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            audio: AudioClient4::new(config.clone())?,
        })
    }

    /// Retrieve the status of the speaker separation process and the list of detected speakers if complete.
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
    ///         .speakers
    ///         .get(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"VW7YKqPnjY4h39yTbx2L".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        voice_id: &str,
        sample_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SpeakerSeparationResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/voices/pvc/{}/samples/{}/speakers", voice_id, sample_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Start speaker separation process for a sample
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
    ///         .speakers
    ///         .separate(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"VW7YKqPnjY4h39yTbx2L".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn separate(
        &self,
        voice_id: &str,
        sample_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<StartSpeakerSeparationResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/voices/pvc/{}/samples/{}/separate-speakers",
                    voice_id, sample_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
