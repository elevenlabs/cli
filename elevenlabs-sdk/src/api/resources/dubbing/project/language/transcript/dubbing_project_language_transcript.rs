use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TranscriptClient3 {
    pub http_client: HttpClient,
}

impl TranscriptClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// A language target's transcript: source segments with their translations. Available once the target has produced an output. Returns a conflict while the target is still on its first dub, since it has no translations to return yet.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project.
    /// * `language_id` - Identifier of the language target.
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
    ///         .language
    ///         .transcript
    ///         .get(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &"lang_1001kwkyxp0je6ktn4knsfrasx5s".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        language_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DubbingTargetTranscriptResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/dubbing/project/{}/language/{}/transcript",
                    project_id, language_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Enterprise only. Edit a segment's translation for a language target. Omitted fields are left unchanged; an explicit null clears the field. Bumps the target's `revision` and marks it `stale` if it had already completed. The source transcript and the project's other languages are untouched, and no audio changes until you regenerate the target.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project.
    /// * `language_id` - Identifier of the language target.
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
    ///         .language
    ///         .transcript
    ///         .update_segment(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &"lang_1001kwkyxp0je6ktn4knsfrasx5s".to_string(),
    ///             &"0199a3f0-1c2d-7abc-8def-0123456789ab".to_string(),
    ///             &DubbingTargetSegmentUpdateRequest {
    ///                 translation: Some(
    ///                     "Bienvenido a nuestra última demostración de producto.".to_string(),
    ///                 ),
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
        language_id: &str,
        segment_id: &str,
        request: &DubbingTargetSegmentUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<DubbingTargetSegmentUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/dubbing/project/{}/language/{}/transcript/segment/{}",
                    project_id, language_id, segment_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Enterprise only. Edit several segments' translations for a language target in one atomic request: every edit applies or none does. Bumps the target's `revision` and marks it `stale` if it had already completed. The source transcript and the project's other languages are untouched, and no audio changes until you regenerate the target.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project.
    /// * `language_id` - Identifier of the language target.
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
    ///         .language
    ///         .transcript
    ///         .update_segments(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &"lang_1001kwkyxp0je6ktn4knsfrasx5s".to_string(),
    ///             &DubbingBulkTargetSegmentUpdateRequest {
    ///                 segments: HashMap::from([
    ///                     (
    ///                         "0199a3f0-1c2d-7abc-8def-0123456789ab".to_string(),
    ///                         DubbingTargetSegmentUpdateRequest {
    ///                             translation: Some(
    ///                                 "Bienvenido a nuestra última demostración de producto.".to_string(),
    ///                             ),
    ///                             ..Default::default()
    ///                         },
    ///                     ),
    ///                     (
    ///                         "0199a3f0-3e4f-7abc-8def-0123456789cd".to_string(),
    ///                         DubbingTargetSegmentUpdateRequest {
    ///                             translation: Some("Empecemos.".to_string()),
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
        language_id: &str,
        request: &DubbingBulkTargetSegmentUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<DubbingBulkTargetSegmentUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/dubbing/project/{}/language/{}/transcript/segments",
                    project_id, language_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Enterprise only. Re-dub a target from its edited transcript, re-synthesizing only the edited regions (charged like a generation, less the free-regeneration allowance). Accepted asynchronously: the target returns to `processing` and sends a `dubbing_language_completed` event to the project's `webhook_ids` when the re-dub lands, carrying the new output URLs. Returns a conflict when the target has no edits to apply — nothing is dispatched and nothing is charged.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project.
    /// * `language_id` - Identifier of the language target.
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
    ///         .language
    ///         .transcript
    ///         .regenerate(
    ///             &"proj_1601kwkyxp0hfzvtmyxwqxx6mcy3".to_string(),
    ///             &"lang_1001kwkyxp0je6ktn4knsfrasx5s".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn regenerate(
        &self,
        project_id: &str,
        language_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DubbingRegenerateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/dubbing/project/{}/language/{}/transcript/regenerate",
                    project_id, language_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
