use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod segment;
pub use segment::SegmentClient2;
pub struct SpeakerClient {
    pub http_client: HttpClient,
    pub segment: SegmentClient2,
}

impl SpeakerClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            segment: SegmentClient2::new(config.clone())?,
        })
    }

    /// Amend the metadata associated with a speaker, such as their voice. Both voice cloning and using voices from the ElevenLabs library are supported.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
    /// * `speaker_id` - ID of the speaker.
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
    ///         .dubbing
    ///         .resource
    ///         .speaker
    ///         .update(
    ///             &"dubbing_id".to_string(),
    ///             &"speaker_id".to_string(),
    ///             &BodyUpdateMetadataForASpeakerV1DubbingResourceDubbingIDSpeakerSpeakerIDPatch {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        dubbing_id: &str,
        speaker_id: &str,
        request: &BodyUpdateMetadataForASpeakerV1DubbingResourceDubbingIdSpeakerSpeakerIdPatch,
        options: Option<RequestOptions>,
    ) -> Result<SpeakerUpdatedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/dubbing/resource/{}/speaker/{}", dubbing_id, speaker_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a new speaker in a dubbing resource. The speaker is added to every available language and can optionally be associated with an ElevenLabs voice and voice settings.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
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
    ///         .dubbing
    ///         .resource
    ///         .speaker
    ///         .create(
    ///             &"dubbing_id".to_string(),
    ///             &BodyCreateANewSpeakerV1DubbingResourceDubbingIDSpeakerPost {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        dubbing_id: &str,
        request: &BodyCreateANewSpeakerV1DubbingResourceDubbingIdSpeakerPost,
        options: Option<RequestOptions>,
    ) -> Result<SpeakerCreatedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/dubbing/resource/{}/speaker", dubbing_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetch the top 10 similar voices to a speaker, including the voice IDs, names, descriptions, and, where possible, a sample audio recording.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
    /// * `speaker_id` - ID of the speaker.
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
    ///         .dubbing
    ///         .resource
    ///         .speaker
    ///         .find_similar_voices(&"dubbing_id".to_string(), &"speaker_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn find_similar_voices(
        &self,
        dubbing_id: &str,
        speaker_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SimilarVoicesForSpeakerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/dubbing/resource/{}/speaker/{}/similar-voices",
                    dubbing_id, speaker_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
