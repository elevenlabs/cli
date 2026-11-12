use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CallsClient {
    pub http_client: HttpClient,
}

impl CallsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the calls of an outbound campaign, newest first: first contacts, retries, callbacks and redirects, whether queued, under way or done.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many items to return at most.
    /// * `lead_id` - Only this lead's calls.
    /// * `status` - Only calls in this status.
    /// * `call_result` - Only calls that ended in one of these results.
    /// * `stage` - Only messages at this delivery stage. Message campaigns only.
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
    ///         .calls
    ///         .list(
    ///             &"campaign_id".to_string(),
    ///             &AgentsOutboundCampaignsCallsListQueryRequest {
    ///                 page_size: Some(1),
    ///                 lead_id: Some("lead_id".to_string()),
    ///                 status: Some(CallStatus::Queued),
    ///                 call_result: vec![Some(CallResult::Answered)],
    ///                 stage: Some(CallsListRequestStage::Queued),
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
        request: &AgentsOutboundCampaignsCallsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CampaignCallsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/outbound-campaigns/campaigns/{}/calls",
                    campaign_id
                ),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("lead_id", request.lead_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize_array("call_result", request.call_result.clone())
                    .serialize("stage", request.stage.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Download every call of a campaign, with its lead and conversation results, as CSV.
    ///
    /// # Arguments
    ///
    /// * `limit` - Only export the first N calls; used to preview the columns. Omit to export every call.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
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
    ///         .calls
    ///         .export(
    ///             &"campaign_id".to_string(),
    ///             &AgentsOutboundCampaignsCallsExportQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export(
        &self,
        campaign_id: &str,
        request: &AgentsOutboundCampaignsCallsExportQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!(
                    "v1/convai/outbound-campaigns/campaigns/{}/export",
                    campaign_id
                ),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }
}
