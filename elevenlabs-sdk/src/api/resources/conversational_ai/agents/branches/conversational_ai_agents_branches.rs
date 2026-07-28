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
    ///         .agents
    ///         .branches
    ///         .list(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &ConversationalAiAgentsBranchesListQueryRequest {
    ///                 include_archived: Some(true),
    ///                 limit: Some(1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .agents
    ///         .branches
    ///         .create(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &BodyCreateANewBranchV1ConvaiAgentsAgentIDBranchesPost {
    ///                 parent_version_id: "parent_version_id".to_string(),
    ///                 name: "name".to_string(),
    ///                 description: "description".to_string(),
    ///                 conversation_config: None,
    ///                 platform_settings: None,
    ///                 workflow: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .agents
    ///         .branches
    ///         .get(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbranch_0901k4aafjxxfxt93gd841r7tv5t".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .agents
    ///         .branches
    ///         .update(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbranch_0901k4aafjxxfxt93gd841r7tv5t".to_string(),
    ///             &BodyUpdateAgentBranchV1ConvaiAgentsAgentIDBranchesBranchIDPatch {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .agents
    ///         .branches
    ///         .preview_merge(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbrch_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///             &PreviewMergeQueryRequest {
    ///                 target_branch_id: "agtbrch_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///                 force: Some(true),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///     client.conversational_ai.agents.branches.merge(&"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(), &"agtbrch_8901k4t9z5defmb8vh3e9361y7nj".to_string(), &BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIDBranchesSourceBranchIDMergePost {
    ///         target_branch_id: "agtbrch_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///         archive_source_branch: None,
    ///         force: None
    ///     }, None).await;
    /// }
    /// ```
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
    ///         .agents
    ///         .branches
    ///         .preview_rebase(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbrch_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .agents
    ///         .branches
    ///         .rebase(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtbrch_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
