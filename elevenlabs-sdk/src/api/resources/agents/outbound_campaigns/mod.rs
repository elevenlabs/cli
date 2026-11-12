use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod config;
pub use config::ConfigClient;
pub mod leads;
pub use leads::LeadsClient;
pub mod calls;
pub use calls::CallsClient;
pub mod attention;
pub use attention::AttentionClient;
pub struct OutboundCampaignsClient {
    pub http_client: HttpClient,
    pub config: ConfigClient,
    pub leads: LeadsClient,
    pub calls: CallsClient,
    pub attention: AttentionClient,
}

impl OutboundCampaignsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            config: ConfigClient::new(config.clone())?,
            leads: LeadsClient::new(config.clone())?,
            calls: CallsClient::new(config.clone())?,
            attention: AttentionClient::new(config.clone())?,
        })
    }

    /// List the workspace's outbound campaigns, newest first.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many items to return at most.
    /// * `status` - Only campaigns in one of these statuses.
    /// * `agent_id` - Only campaigns run by this agent.
    /// * `channel` - Only campaigns on this channel.
    /// * `mode` - Only voice or only message campaigns.
    /// * `search` - Only campaigns whose name contains this text.
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
    ///         .list(
    ///             &AgentsOutboundCampaignsListQueryRequest {
    ///                 page_size: Some(1),
    ///                 status: vec![Some(CampaignStatus::Active)],
    ///                 agent_id: Some("agent_id".to_string()),
    ///                 channel: Some(OutboundChannel::Phone),
    ///                 mode: Some(OutboundCampaignsListRequestMode::Voice),
    ///                 search: Some("search".to_string()),
    ///                 cursor: Some("cursor".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AgentsOutboundCampaignsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CampaignPageResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/outbound-campaigns/campaigns",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .serialize_array("status", request.status.clone())
                    .string("agent_id", request.agent_id.clone())
                    .serialize("channel", request.channel.clone())
                    .serialize("mode", request.mode.clone())
                    .string("search", request.search.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Pause every running outbound campaign in the workspace. Paused campaigns place no new calls until resumed.
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
    ///     client.agents.outbound_campaigns.pause_all(None).await;
    /// }
    /// ```
    pub async fn pause_all(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<PauseAllCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/outbound-campaigns/campaigns/pause-all",
                None,
                None,
                options,
            )
            .await
    }

    /// Stop every outbound campaign in the workspace that hasn't finished. Returns `202` while calls already placed are still finishing.
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
    ///     client.agents.outbound_campaigns.stop_all(None).await;
    /// }
    /// ```
    pub async fn stop_all(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<StopAllCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/outbound-campaigns/campaigns/stop-all",
                None,
                None,
                options,
            )
            .await
    }

    /// Create an outbound campaign that calls or messages each recipient with the agent, within the policy's calling windows, retrying and following up as it allows. Returns `201` for a new campaign. A repeated request, matched by `idempotency_key` or else by its body, returns the existing campaign with `created` false.
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
    ///         .create(
    ///             &SubmitCampaignRequest {
    ///                 call_name: "call_name".to_string(),
    ///                 agent_id: "agent_id".to_string(),
    ///                 recipients: vec![SubmitCampaignRecipient {
    ///                     ..Default::default()
    ///                 }],
    ///                 channel_config: SubmitCampaignRequestChannelConfig::Phone {
    ///                     data: PhoneChannelConfigInput {
    ///                         agent_phone_number_id: "agent_phone_number_id".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                 },
    ///                 scheduled_time_unix: None,
    ///                 timezone: None,
    ///                 branch_id: None,
    ///                 environment: None,
    ///                 target_concurrency_limit: None,
    ///                 idempotency_key: None,
    ///                 policy: None,
    ///                 variant: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &SubmitCampaignRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubmitCampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/outbound-campaigns/campaigns/submit",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get an outbound campaign and its lead counts.
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
    ///         .get(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/outbound-campaigns/campaigns/{}", campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get where an outbound campaign's leads and calls stand, and when its next call is due.
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
    ///         .get_progress(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_progress(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignProgressResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/outbound-campaigns/campaigns/{}/progress",
                    campaign_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Count the callbacks and redirects contacts asked for during an outbound campaign's calls.
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
    ///         .get_follow_up_requests(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_follow_up_requests(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignFollowUpRequestsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/outbound-campaigns/campaigns/{}/follow-up-requests",
                    campaign_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Pause an outbound campaign. It places no new calls until resumed; calls already placed carry on.
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
    ///         .pause(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn pause(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/outbound-campaigns/campaigns/{}/pause",
                    campaign_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Resume a paused outbound campaign, including calls an attention item held back.
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
    ///         .resume(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn resume(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/outbound-campaigns/campaigns/{}/resume",
                    campaign_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Stop an outbound campaign for good: queued calls are cancelled and no lead is called again. Returns `202` with the campaign `stopping` while calls already placed are still finishing.
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
    ///         .stop(&"campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn stop(
        &self,
        campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CampaignResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/outbound-campaigns/campaigns/{}/stop",
                    campaign_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
