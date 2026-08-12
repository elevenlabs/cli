use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod language;
pub use language::LanguageClient2;
pub mod segment;
pub use segment::SegmentClient;
pub mod speaker;
pub use speaker::SpeakerClient;
pub struct ResourceClient {
    pub http_client: HttpClient,
    pub language: LanguageClient2,
    pub segment: SegmentClient,
    pub speaker: SpeakerClient,
}

impl ResourceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            language: LanguageClient2::new(config.clone())?,
            segment: SegmentClient::new(config.clone())?,
            speaker: SpeakerClient::new(config.clone())?,
        })
    }

    /// Given a dubbing ID generated from the '/v1/dubbing' endpoint with studio enabled, returns the dubbing resource.
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
    ///         .get(&"dubbing_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        dubbing_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DubbingResource, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/dubbing/resource/{}", dubbing_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Change the attribution of one or more segments to a different speaker.
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
    ///         .migrate_segments(
    ///             &"dubbing_id".to_string(),
    ///             &BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIDMigrateSegmentsPost {
    ///                 segment_ids: vec!["segment_ids".to_string()],
    ///                 speaker_id: "speaker_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn migrate_segments(
        &self,
        dubbing_id: &str,
        request: &BodyMoveSegmentsBetweenSpeakersV1DubbingResourceDubbingIdMigrateSegmentsPost,
        options: Option<RequestOptions>,
    ) -> Result<SegmentMigrationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/dubbing/resource/{}/migrate-segments", dubbing_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Regenerate the transcriptions for the specified segments. Does not automatically regenerate translations or dubs.
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
    ///         .transcribe(
    ///             &"dubbing_id".to_string(),
    ///             &BodyTranscribesSegmentsV1DubbingResourceDubbingIDTranscribePost {
    ///                 segments: vec!["segments".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn transcribe(
        &self,
        dubbing_id: &str,
        request: &BodyTranscribesSegmentsV1DubbingResourceDubbingIdTranscribePost,
        options: Option<RequestOptions>,
    ) -> Result<SegmentTranscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/dubbing/resource/{}/transcribe", dubbing_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Regenerate the translations for either the entire resource or the specified segments/languages. Will automatically transcribe missing transcriptions. Will not automatically regenerate the dubs.
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
    ///         .translate(
    ///             &"dubbing_id".to_string(),
    ///             &BodyTranslatesAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIDTranslatePost {
    ///                 segments: vec!["segments".to_string()],
    ///                 languages: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn translate(
        &self,
        dubbing_id: &str,
        request: &BodyTranslatesAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdTranslatePost,
        options: Option<RequestOptions>,
    ) -> Result<SegmentTranslationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/dubbing/resource/{}/translate", dubbing_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Regenerate the dubs for either the entire resource or the specified segments/languages. Will automatically transcribe and translate any missing transcriptions and translations.
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
    ///         .dub(
    ///             &"dubbing_id".to_string(),
    ///             &BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIDDubPost {
    ///                 segments: vec!["segments".to_string()],
    ///                 languages: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn dub(
        &self,
        dubbing_id: &str,
        request: &BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPost,
        options: Option<RequestOptions>,
    ) -> Result<SegmentDubResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/dubbing/resource/{}/dub", dubbing_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Regenerate the output media for a language using the latest Studio state. Please ensure all segments have been dubbed before rendering, otherwise they will be omitted. Renders are generated asynchronously, and to check the status of all renders please use the 'Get Dubbing Resource' endpoint.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
    /// * `language` - The target language code to render, eg. 'es'. To render the source track use 'original'.
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
    ///     client.dubbing.resource.render(&"dubbing_id".to_string(), &"language".to_string(), &BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIDRenderLanguagePost {
    ///         render_type: RenderType::Mp4,
    ///         normalize_volume: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn render(
        &self,
        dubbing_id: &str,
        language: &str,
        request: &BodyRenderAudioOrVideoForTheGivenLanguageV1DubbingResourceDubbingIdRenderLanguagePost,
        options: Option<RequestOptions>,
    ) -> Result<DubbingRenderResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/dubbing/resource/{}/render/{}", dubbing_id, language),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
