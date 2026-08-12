use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ToolApprovalsClient {
    pub http_client: HttpClient,
}

impl ToolApprovalsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Add approval for a specific MCP tool when using per-tool approval mode.
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
    ///         .agents
    ///         .mcp_servers
    ///         .tool_approvals
    ///         .create(
    ///             &"mcp_server_id".to_string(),
    ///             &McpToolAddApprovalRequestModel {
    ///                 tool_name: "tool_name".to_string(),
    ///                 tool_description: "tool_description".to_string(),
    ///                 input_schema: None,
    ///                 approval_policy: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        mcp_server_id: &str,
        request: &McpToolAddApprovalRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<McpServerResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/mcp-servers/{}/tool-approvals", mcp_server_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove approval for a specific MCP tool when using per-tool approval mode.
    ///
    /// # Arguments
    ///
    /// * `mcp_server_id` - ID of the MCP Server.
    /// * `tool_name` - Name of the MCP tool to remove approval for.
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
    ///         .tool_approvals
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
                    "v1/convai/mcp-servers/{}/tool-approvals/{}",
                    mcp_server_id, tool_name
                ),
                None,
                None,
                options,
            )
            .await
    }
}
