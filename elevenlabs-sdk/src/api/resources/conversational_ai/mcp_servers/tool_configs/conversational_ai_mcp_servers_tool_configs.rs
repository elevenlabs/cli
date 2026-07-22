use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ToolConfigsClient {
    pub http_client: HttpClient,
}

impl ToolConfigsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create configuration overrides for a specific MCP tool.
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
    ///         .conversational_ai
    ///         .mcp_servers
    ///         .tool_configs
    ///         .create(
    ///             &"mcp_server_id".to_string(),
    ///             &McpToolConfigOverrideCreateRequestModel {
    ///                 environment: Some("environment".to_string()),
    ///                 tool_name: "tool_name".to_string(),
    ///                 force_pre_tool_speech: None,
    ///                 pre_tool_speech: None,
    ///                 disable_interruptions: None,
    ///                 interruption_mode: None,
    ///                 tool_call_sound: None,
    ///                 tool_call_sound_behavior: None,
    ///                 execution_mode: None,
    ///                 response_timeout_secs: None,
    ///                 assignments: None,
    ///                 input_overrides: None,
    ///                 response_mocks: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        mcp_server_id: &str,
        request: &McpToolConfigOverrideCreateRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<McpServerResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/mcp-servers/{}/tool-configs", mcp_server_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve configuration overrides for a specific MCP tool.
    ///
    /// # Arguments
    ///
    /// * `mcp_server_id` - ID of the MCP Server.
    /// * `tool_name` - Name of the MCP tool to retrieve config overrides for.
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
    ///         .conversational_ai
    ///         .mcp_servers
    ///         .tool_configs
    ///         .get(&"mcp_server_id".to_string(), &"tool_name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        mcp_server_id: &str,
        tool_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<McpToolConfigOverrideOutput, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/mcp-servers/{}/tool-configs/{}",
                    mcp_server_id, tool_name
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Remove configuration overrides for a specific MCP tool.
    ///
    /// # Arguments
    ///
    /// * `mcp_server_id` - ID of the MCP Server.
    /// * `tool_name` - Name of the MCP tool to remove config overrides for.
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
    ///         .conversational_ai
    ///         .mcp_servers
    ///         .tool_configs
    ///         .delete(&"mcp_server_id".to_string(), &"tool_name".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        mcp_server_id: &str,
        tool_name: &str,
        options: Option<RequestOptions>,
    ) -> Result<McpServerResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/convai/mcp-servers/{}/tool-configs/{}",
                    mcp_server_id, tool_name
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Update configuration overrides for a specific MCP tool.
    ///
    /// # Arguments
    ///
    /// * `mcp_server_id` - ID of the MCP Server.
    /// * `tool_name` - Name of the MCP tool to update config overrides for.
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
    ///         .conversational_ai
    ///         .mcp_servers
    ///         .tool_configs
    ///         .update(
    ///             &"mcp_server_id".to_string(),
    ///             &"tool_name".to_string(),
    ///             &McpToolConfigOverrideUpdateRequestModel {
    ///                 environment: Some("environment".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        mcp_server_id: &str,
        tool_name: &str,
        request: &McpToolConfigOverrideUpdateRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<McpServerResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/convai/mcp-servers/{}/tool-configs/{}",
                    mcp_server_id, tool_name
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }
}
