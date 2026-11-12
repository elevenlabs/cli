use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LeadsClient {
    pub http_client: HttpClient,
}

impl LeadsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the leads of an outbound campaign, one per recipient.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many items to return at most.
    /// * `search` - Only leads whose ID, user ID or phone number contains this text.
    /// * `status` - `in_progress` for leads still being called, `finished` for leads that won't be called again.
    /// * `disposition` - Only leads that finished in one of these ways.
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
    ///         .leads
    ///         .list(
    ///             &"campaign_id".to_string(),
    ///             &AgentsOutboundCampaignsLeadsListQueryRequest {
    ///                 page_size: Some(1),
    ///                 search: Some("search".to_string()),
    ///                 status: Some(LeadsListRequestStatus::InProgress),
    ///                 disposition: vec![Some(TerminalDisposition::CampaignEnded)],
    ///                 cursor: Some("cursor".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        campaign_id: &str,
        request: &AgentsOutboundCampaignsLeadsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CampaignLeadsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/outbound-campaigns/campaigns/{}/leads",
                    campaign_id
                ),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("search", request.search.clone())
                    .serialize("status", request.status.clone())
                    .serialize_array("disposition", request.disposition.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get one lead of an outbound campaign.
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
    ///         .leads
    ///         .get(&"campaign_id".to_string(), &"lead_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        campaign_id: &str,
        lead_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<LeadDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/outbound-campaigns/campaigns/{}/leads/{}",
                    campaign_id, lead_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Remove a lead from an outbound campaign so it is never called again. Returns `202` when a call already placed may still go through.
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
    ///         .leads
    ///         .delete(&"campaign_id".to_string(), &"lead_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        campaign_id: &str,
        lead_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SuppressLeadResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/convai/outbound-campaigns/campaigns/{}/leads/{}",
                    campaign_id, lead_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
