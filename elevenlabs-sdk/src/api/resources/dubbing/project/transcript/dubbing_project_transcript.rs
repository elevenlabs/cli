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

    /// Remove a source segment from the transcript.
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

    /// Edit a source segment's text, speaker, or timing.
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

    /// Add a new source segment to the transcript.
    ///
    /// # Arguments
    ///
    /// * `project_id` - Identifier of the dubbing project.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
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
