use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CompositionPlanClient {
    pub http_client: HttpClient,
}

impl CompositionPlanClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a composition plan for music generation. Usage of this endpoint does not cost any credits but is subject to rate limiting depending on your tier.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &BodyGenerateCompositionPlanV1MusicPlanPost,
        options: Option<RequestOptions>,
    ) -> Result<CompositionPlanCreateResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/music/plan",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
