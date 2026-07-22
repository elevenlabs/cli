use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BranchesClient {
    pub http_client: HttpClient,
}

impl BranchesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a list of branches an agent has
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `include_archived` - Whether archived branches should be included
    /// * `limit` - How many results at most should be returned
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        agent_id: &str,
        request: &ConversationalAiAgentsBranchesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListResponseAgentBranchSummary, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agents/{}/branches", agent_id),
                None,
                QueryBuilder::new()
                    .bool("include_archived", request.include_archived.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new branch from a given version of any branch
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        agent_id: &str,
        request: &BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPost,
        options: Option<RequestOptions>,
    ) -> Result<CreateAgentBranchResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agents/{}/branches", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a single agent branch
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `branch_id` - Unique identifier for the branch.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        agent_id: &str,
        branch_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AgentBranchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agents/{}/branches/{}", agent_id, branch_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update agent branch properties such as archiving status and protection level
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `branch_id` - Unique identifier for the branch.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        agent_id: &str,
        branch_id: &str,
        request: &BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch,
        options: Option<RequestOptions>,
    ) -> Result<AgentBranchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/agents/{}/branches/{}", agent_id, branch_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the result of merging the source branch into the target branch without performing the merge. Useful for showing an accurate diff before confirming.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `source_branch_id` - Unique identifier for the source branch to merge from.
    /// * `target_branch_id` - The ID of the target branch to merge into.
    /// * `force` - When true, source branch changes always win conflicts regardless of timestamps
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn preview_merge(
        &self,
        agent_id: &str,
        source_branch_id: &str,
        request: &PreviewMergeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MergePreviewResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/agents/{}/branches/{}/merge-preview",
                    agent_id, source_branch_id
                ),
                None,
                QueryBuilder::new()
                    .string("target_branch_id", request.target_branch_id.clone())
                    .bool("force", request.force.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Merge a branch into a target branch
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `source_branch_id` - Unique identifier for the source branch to merge from.
    /// * `target_branch_id` - The ID of the target branch to merge into.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn merge(
        &self,
        agent_id: &str,
        source_branch_id: &str,
        request: &BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/agents/{}/branches/{}/merge",
                    agent_id, source_branch_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("target_branch_id", request.target_branch_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the result of rebasing the branch onto main without performing the rebase. Useful for showing an accurate diff before confirming.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `branch_id` - Unique identifier for the source branch to merge from.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn preview_rebase(
        &self,
        agent_id: &str,
        branch_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MergePreviewResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/agents/{}/branches/{}/rebase-preview",
                    agent_id, branch_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Rebase a branch onto the latest main branch, incorporating main's changes while preserving the branch's own changes.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `branch_id` - Unique identifier for the source branch to merge from.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn rebase(
        &self,
        agent_id: &str,
        branch_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/agents/{}/branches/{}/rebase",
                    agent_id, branch_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
