use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SegmentClient2 {
    pub http_client: HttpClient,
}

impl SegmentClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a new segment in dubbing resource with a start and end time for the speaker in every available language. Does not automatically generate transcripts/translations/audio.
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
    ///         .segment
    ///         .create(
    ///             &"dubbing_id".to_string(),
    ///             &"speaker_id".to_string(),
    ///             &SegmentCreatePayload {
    ///                 start_time: 1.1,
    ///                 end_time: 1.1,
    ///                 text: None,
    ///                 translations: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        dubbing_id: &str,
        speaker_id: &str,
        request: &SegmentCreatePayload,
        options: Option<RequestOptions>,
    ) -> Result<SegmentCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/dubbing/resource/{}/speaker/{}/segment",
                    dubbing_id, speaker_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
