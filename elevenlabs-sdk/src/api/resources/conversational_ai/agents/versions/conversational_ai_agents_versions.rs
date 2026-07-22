use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct VersionsClient {
    pub http_client: HttpClient,
}

impl VersionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get metadata for a specific agent version
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `version_id` - Unique identifier for the version.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        agent_id: &str,
        version_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AgentVersionMetadata, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agents/{}/versions/{}", agent_id, version_id),
                None,
                None,
                options,
            )
            .await
    }
}
