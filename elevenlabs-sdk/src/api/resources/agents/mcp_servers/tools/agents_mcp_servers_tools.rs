use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ToolsClient2 {
    pub http_client: HttpClient,
}

impl ToolsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve all tools available for a specific MCP server configuration.
    ///
    /// # Arguments
    ///
    /// * `mcp_server_id` - ID of the MCP Server.
    /// * `environment` - Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production.
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
    ///         .mcp_servers
    ///         .tools
    ///         .list(
    ///             &"mcp_server_id".to_string(),
    ///             &AgentsMcpServersToolsListQueryRequest {
    ///                 environment: Some("environment".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        mcp_server_id: &str,
        request: &AgentsMcpServersToolsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMcpToolsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/mcp-servers/{}/tools", mcp_server_id),
                None,
                QueryBuilder::new()
                    .string("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }
}
