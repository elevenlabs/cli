use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DependentAgentsClient {
    pub http_client: HttpClient,
}

impl DependentAgentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a list of agents (and branches) that reference this analysis item.
    ///
    /// # Arguments
    ///
    /// * `analysis_item_id` - ID of the requested analysis item.
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100, defaults to 30.
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
    ///         .analysis_items
    ///         .dependent_agents
    ///         .get(
    ///             &"analysis_item_id".to_string(),
    ///             &AgentsAnalysisItemsDependentAgentsGetQueryRequest {
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        analysis_item_id: &str,
        request: &AgentsAnalysisItemsDependentAgentsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAnalysisItemDependentAgentsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/analysis-items/{}/dependent-agents",
                    analysis_item_id
                ),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
