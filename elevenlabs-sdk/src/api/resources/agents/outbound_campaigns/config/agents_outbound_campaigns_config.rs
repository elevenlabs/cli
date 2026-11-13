use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ConfigClient {
    pub http_client: HttpClient,
}

impl ConfigClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get the workspace's outbound campaign configuration: the default policy new campaigns take, and per-agent overrides.
    ///
    /// # Arguments
    ///
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
    ///     client.agents.outbound_campaigns.config.get(None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceConfigResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/outbound-campaigns/config",
                None,
                None,
                options,
            )
            .await
    }

    /// Create or replace the workspace's outbound campaign configuration. Send the `version` you last read as `expected_version`, or omit it to create the first configuration. A configuration changed since you read it is not replaced. Campaigns keep the policy they started with.
    ///
    /// # Arguments
    ///
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
    ///         .agents
    ///         .outbound_campaigns
    ///         .config
    ///         .update(
    ///             &ReplaceWorkspaceConfigRequest {
    ///                 config: CampaignConfigInput {
    ///                     ..Default::default()
    ///                 },
    ///                 expected_version: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        request: &ReplaceWorkspaceConfigRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceConfigResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "v1/convai/outbound-campaigns/config",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
