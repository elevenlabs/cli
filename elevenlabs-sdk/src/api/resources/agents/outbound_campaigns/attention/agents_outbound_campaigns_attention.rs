use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AttentionClient {
    pub http_client: HttpClient,
}

impl AttentionClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List problems in the workspace's outbound campaigns that need you to act, such as an agent or phone number that can no longer be used, newest first. A campaign with an open item holds back the calls it affects.
    ///
    /// # Arguments
    ///
    /// * `campaign_id` - Only this campaign's items.
    /// * `include_resolved` - Include items that are already resolved.
    /// * `page_size` - How many items to return at most.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
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
    ///         .attention
    ///         .list(
    ///             &AgentsOutboundCampaignsAttentionListQueryRequest {
    ///                 campaign_id: Some("campaign_id".to_string()),
    ///                 include_resolved: Some(true),
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AgentsOutboundCampaignsAttentionListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AttentionPageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/outbound-campaigns/attention",
                None,
                QueryBuilder::new()
                    .string("campaign_id", request.campaign_id.clone())
                    .bool("include_resolved", request.include_resolved.clone())
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Place the calls a retryable attention item holds back, once its cause is fixed. Resumes the campaign.
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
    ///         .attention
    ///         .retry(&"attention_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retry(
        &self,
        attention_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AttentionView, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/outbound-campaigns/attention/{}/retry",
                    attention_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Close an attention item that can't be retried, recording how it was handled.
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
    ///         .attention
    ///         .resolve(
    ///             &"attention_id".to_string(),
    ///             &AttentionResolutionRequest {
    ///                 resolution: "resolution".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resolve(
        &self,
        attention_id: &str,
        request: &AttentionResolutionRequest,
        options: Option<RequestOptions>,
    ) -> Result<AttentionView, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/outbound-campaigns/attention/{}/resolve",
                    attention_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
