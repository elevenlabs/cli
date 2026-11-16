use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SubagentsClient {
    pub http_client: HttpClient,
}

impl SubagentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a list of agents that use this agent as a subagent.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `page_size` - How many Agents to return at maximum. Can not exceed 100, defaults to 30.
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
    ///         .subagents
    ///         .get_dependents(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &GetDependentsQueryRequest {
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_dependents(
        &self,
        agent_id: &str,
        request: &GetDependentsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentSubagentDependentsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agents/{}/subagent-dependents", agent_id),
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
