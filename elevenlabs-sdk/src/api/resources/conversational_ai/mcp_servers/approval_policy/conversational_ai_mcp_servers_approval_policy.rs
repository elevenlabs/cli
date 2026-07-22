use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ApprovalPolicyClient {
    pub http_client: HttpClient,
}

impl ApprovalPolicyClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Update the approval policy configuration for an MCP server. DEPRECATED: Use PATCH /mcp-servers/{id} endpoint instead.
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
        request: &McpApprovalPolicyUpdateRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<McpServerResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/mcp-servers/{}/approval-policy", mcp_server_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
