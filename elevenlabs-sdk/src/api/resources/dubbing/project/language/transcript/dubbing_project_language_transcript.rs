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

    /// A language target's transcript: source segments with their translations.
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

    /// Edit a segment's translation for a language target.
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

    /// Re-dub a target from its edited transcript (charged like a generation).
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
    pub async fn regenerate(
        &self,
        project_id: &str,
        language_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DubbingLanguageResponse, ApiError> {
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
