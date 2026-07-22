use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod audio;
pub use audio::AudioClient3;
pub mod waveform;
pub use waveform::WaveformClient;
pub mod speakers;
pub use speakers::SpeakersClient;
pub struct SamplesClient2 {
    pub http_client: HttpClient,
    pub audio: AudioClient3,
    pub waveform: WaveformClient,
    pub speakers: SpeakersClient,
}

impl SamplesClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            audio: AudioClient3::new(config.clone())?,
            waveform: WaveformClient::new(config.clone())?,
            speakers: SpeakersClient::new(config.clone())?,
        })
    }

    /// Add audio samples to a PVC voice
    ///
    /// # Arguments
    ///
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
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
    ///         .create(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &CreateRequest {
    ///                 files: vec![b"test file 1".to_vec(), b"test file 2".to_vec()],
    ///                 remove_background_noise: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        voice_id: &str,
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<VoiceSample>, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("v1/voices/pvc/{}/samples", voice_id),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Update a PVC voice sample - apply noise removal, select speaker, change trim times or file name.
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
    ///         .update(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"VW7YKqPnjY4h39yTbx2L".to_string(),
    ///             &BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIDSamplesSampleIDPost {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        voice_id: &str,
        sample_id: &str,
        request: &BodyUpdatePvcVoiceSampleV1VoicesPvcVoiceIdSamplesSampleIdPost,
        options: Option<RequestOptions>,
    ) -> Result<AddVoiceResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/voices/pvc/{}/samples/{}", voice_id, sample_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a sample from a PVC voice.
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
    ///         .delete(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"VW7YKqPnjY4h39yTbx2L".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        voice_id: &str,
        sample_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteVoiceSampleResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/voices/pvc/{}/samples/{}", voice_id, sample_id),
                None,
                None,
                options,
            )
            .await
    }
}
