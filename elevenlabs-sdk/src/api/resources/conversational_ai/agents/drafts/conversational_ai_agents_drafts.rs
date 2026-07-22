use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DraftsClient {
    pub http_client: HttpClient,
}

impl DraftsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Create a new draft for an agent
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `branch_id` - The ID of the agent branch to use
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        agent_id: &str,
        request: &BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPost,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agents/{}/drafts", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("branch_id", request.branch_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a draft for an agent
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `branch_id` - The ID of the agent branch to use
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        agent_id: &str,
        request: &ConversationalAiAgentsDraftsDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/agents/{}/drafts", agent_id),
                None,
                QueryBuilder::new()
                    .string("branch_id", request.branch_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
