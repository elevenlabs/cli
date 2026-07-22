use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SegmentClient {
    pub http_client: HttpClient,
}

impl SegmentClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Modifies a single segment with new text and/or start/end times. Will update the values for only a specific language of a segment. Does not automatically regenerate the dub.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
    /// * `segment_id` - ID of the segment
    /// * `language` - ID of the language.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        dubbing_id: &str,
        segment_id: &str,
        language: &str,
        request: &SegmentUpdatePayload,
        options: Option<RequestOptions>,
    ) -> Result<SegmentUpdateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/dubbing/resource/{}/segment/{}/{}",
                    dubbing_id, segment_id, language
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a single segment from the dubbing.
    ///
    /// # Arguments
    ///
    /// * `dubbing_id` - ID of the dubbing project.
    /// * `segment_id` - ID of the segment
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        dubbing_id: &str,
        segment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SegmentDeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/dubbing/resource/{}/segment/{}", dubbing_id, segment_id),
                None,
                None,
                options,
            )
            .await
    }
}
