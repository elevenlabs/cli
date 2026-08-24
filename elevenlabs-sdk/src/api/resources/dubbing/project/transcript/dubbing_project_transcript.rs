use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TranscriptClient2 {
    pub http_client: HttpClient,
}

impl TranscriptClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// The project's source transcript, as editable segments.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project.
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
    ///         .project
    ///         .transcript
    ///         .get(&"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DubbingSourceTranscriptResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/dubbing/project/{}/transcript", project_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Enterprise only. Remove a source segment from the transcript.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project.
    /// * `segment_id` - Identifier of the segment to remove.
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
    ///         .project
    ///         .transcript
    ///         .delete_segment(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &"0199a3f0-1c2d-7abc-8def-0123456789ab".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_segment(
        &self,
        project_id: &str,
        segment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DubbingTranscriptRevisionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/dubbing/project/{}/transcript/segment/{}",
                    project_id, segment_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Enterprise only. Edit a source segment's text, speaker, or timing.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project.
    /// * `segment_id` - Identifier of the segment to edit.
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
    ///         .project
    ///         .transcript
    ///         .update_segment(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &"0199a3f0-1c2d-7abc-8def-0123456789ab".to_string(),
    ///             &DubbingSegmentUpdateRequest {
    ///                 text: Some("Welcome to our latest product demo.".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_segment(
        &self,
        project_id: &str,
        segment_id: &str,
        request: &DubbingSegmentUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<DubbingSourceSegmentUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/dubbing/project/{}/transcript/segment/{}",
                    project_id, segment_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Enterprise only. Edit several source segments' text, speaker, or timing in one atomic request.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project.
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
    ///         .project
    ///         .transcript
    ///         .update_segments(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &DubbingBulkSegmentUpdateRequest {
    ///                 segments: HashMap::from([
    ///                     (
    ///                         "0199a3f0-1c2d-7abc-8def-0123456789ab".to_string(),
    ///                         DubbingSegmentUpdateRequest {
    ///                             text: Some("Welcome to our latest product demo.".to_string()),
    ///                             ..Default::default()
    ///                         },
    ///                     ),
    ///                     (
    ///                         "0199a3f0-3e4f-7abc-8def-0123456789cd".to_string(),
    ///                         DubbingSegmentUpdateRequest {
    ///                             speaker_id: Some("narrator".to_string()),
    ///                             ..Default::default()
    ///                         },
    ///                     ),
    ///                 ]),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_segments(
        &self,
        project_id: &str,
        request: &DubbingBulkSegmentUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<DubbingBulkSourceSegmentUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/dubbing/project/{}/transcript/segments", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Enterprise only. Add a new source segment to the transcript.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project.
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
    ///         .project
    ///         .transcript
    ///         .create_segment(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &DubbingSegmentCreateRequest {
    ///                 text: "Thanks for watching.".to_string(),
    ///                 speaker_id: "default_speaker".to_string(),
    ///                 start_s: 42.0,
    ///                 end_s: 44.0,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_segment(
        &self,
        project_id: &str,
        request: &DubbingSegmentCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<DubbingSourceSegmentUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/dubbing/project/{}/transcript/segment", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
