use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod projects;
pub use projects::ProjectsClient;
pub struct StudioClient {
    pub http_client: HttpClient,
    pub projects: ProjectsClient,
}

impl StudioClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            projects: ProjectsClient::new(config.clone())?,
        })
    }

    /// Create and auto-convert a podcast project. Currently, the LLM cost is covered by us but you will still be charged for the audio generation. In the future, you will be charged for both the LLM and audio generation costs.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_podcast(
        &self,
        request: &BodyCreatePodcastV1StudioPodcastsPost,
        options: Option<RequestOptions>,
    ) -> Result<PodcastProjectResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/studio/podcasts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
