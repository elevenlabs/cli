use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod samples;
pub use samples::SamplesClient2;
pub mod verification;
pub use verification::VerificationClient;
pub struct PvcClient {
    pub http_client: HttpClient,
    pub samples: SamplesClient2,
    pub verification: VerificationClient,
}

impl PvcClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            samples: SamplesClient2::new(config.clone())?,
            verification: VerificationClient::new(config.clone())?,
        })
    }

    /// Creates a new PVC voice with metadata but no samples
    ///
    /// # Arguments
    ///
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
    ///         .create(
    ///             &CreatePvcVoiceRequest {
    ///                 name: "John Smith".to_string(),
    ///                 language: "en".to_string(),
    ///                 description: None,
    ///                 labels: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePvcVoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddVoiceResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/voices/pvc",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Edit PVC voice metadata
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
    ///         .update(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &BodyEditPvcVoiceV1VoicesPvcVoiceIDPost {
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
        request: &BodyEditPvcVoiceV1VoicesPvcVoiceIdPost,
        options: Option<RequestOptions>,
    ) -> Result<AddVoiceResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/voices/pvc/{}", voice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Start PVC training process for a voice.
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
    ///         .train(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &BodyRunPvcTrainingV1VoicesPvcVoiceIDTrainPost {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn train(
        &self,
        voice_id: &str,
        request: &BodyRunPvcTrainingV1VoicesPvcVoiceIdTrainPost,
        options: Option<RequestOptions>,
    ) -> Result<StartPvcVoiceTrainingResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/voices/pvc/{}/train", voice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
