use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod tools;
pub use tools::ToolsClient2;
pub mod approval_policy;
pub use approval_policy::ApprovalPolicyClient;
pub mod tool_approvals;
pub use tool_approvals::ToolApprovalsClient;
pub mod tool_configs;
pub use tool_configs::ToolConfigsClient;
pub struct McpServersClient {
    pub http_client: HttpClient,
    pub tools: ToolsClient2,
    pub approval_policy: ApprovalPolicyClient,
    pub tool_approvals: ToolApprovalsClient,
    pub tool_configs: ToolConfigsClient,
}

impl McpServersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            tools: ToolsClient2::new(config.clone())?,
            approval_policy: ApprovalPolicyClient::new(config.clone())?,
            tool_approvals: ToolApprovalsClient::new(config.clone())?,
            tool_configs: ToolConfigsClient::new(config.clone())?,
        })
    }

    /// Retrieve all MCP server configurations available in the workspace.
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
    ///     client.conversational_ai.mcp_servers.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<McpServersResponseModel, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/convai/mcp-servers", None, None, options)
            .await
    }

    /// Create a new MCP server configuration in the workspace.
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
    ///         .conversational_ai
    ///         .mcp_servers
    ///         .create(
    ///             &McpServerRequestModel {
    ///                 config: McpServerConfigInput {
    ///                     approval_policy: None,
    ///                     tool_approval_hashes: None,
    ///                     transport: None,
    ///                     url: McpServerConfigInputURL::String("url".to_string()),
    ///                     secret_token: None,
    ///                     request_headers: None,
    ///                     auth_connection: None,
    ///                     name: "name".to_string(),
    ///                     description: None,
    ///                     force_pre_tool_speech: None,
    ///                     pre_tool_speech: None,
    ///                     disable_interruptions: None,
    ///                     interruption_mode: None,
    ///                     tool_call_sound: None,
    ///                     tool_call_sound_behavior: None,
    ///                     execution_mode: None,
    ///                     response_timeout_secs: None,
    ///                     tool_config_overrides: None,
    ///                     disable_compression: None,
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &McpServerRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<McpServerResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/mcp-servers",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a specific MCP server configuration from the workspace.
    ///
    /// # Arguments
    ///
    /// * `mcp_server_id` - ID of the MCP Server.
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
    ///         .get(&"mcp_server_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        mcp_server_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<McpServerResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/mcp-servers/{}", mcp_server_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a specific MCP server configuration from the workspace.
    ///
    /// # Arguments
    ///
    /// * `mcp_server_id` - ID of the MCP Server.
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
    ///         .delete(&"mcp_server_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        mcp_server_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/mcp-servers/{}", mcp_server_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the configuration settings for an MCP server.
    ///
    /// # Arguments
    ///
    /// * `mcp_server_id` - ID of the MCP Server.
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
    ///         .update(
    ///             &"mcp_server_id".to_string(),
    ///             &McpServerConfigUpdateRequestModel {
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
        request: &McpServerConfigUpdateRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<McpServerResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/mcp-servers/{}", mcp_server_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
