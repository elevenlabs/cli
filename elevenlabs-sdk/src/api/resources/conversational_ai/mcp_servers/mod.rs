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
