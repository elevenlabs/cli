use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MergeProposalsClient {
    pub http_client: HttpClient,
}

impl MergeProposalsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List the proposals for an agent, newest first.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `status` - Only return proposals with this status.
    /// * `source_branch_id` - Only return proposals with this source branch.
    /// * `target_branch_id` - Only return proposals with this target branch.
    /// * `search` - Case-insensitive substring match over title and description.
    /// * `page_size` - How many results at most should be returned
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
    ///         .merge_proposals
    ///         .list(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &AgentsMergeProposalsListQueryRequest {
    ///                 status: Some(MergeProposalStatus::Open),
    ///                 source_branch_id: Some("source_branch_id".to_string()),
    ///                 target_branch_id: Some("target_branch_id".to_string()),
    ///                 search: Some("search".to_string()),
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
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
        request: &AgentsMergeProposalsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaginatedResultAgentMergeProposalResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agents/{}/merge-proposals", agent_id),
                None,
                QueryBuilder::new()
                    .serialize("status", request.status.clone())
                    .string("source_branch_id", request.source_branch_id.clone())
                    .string("target_branch_id", request.target_branch_id.clone())
                    .string("search", request.search.clone())
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Record a request to merge a source branch into a target branch. Anyone with edit access can open one; merging it later is gated on write access to the target branch, so this is how a change reaches a protected branch the author cannot merge into themselves.
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
    ///         .agents
    ///         .merge_proposals
    ///         .create(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &BodyCreateAMergeProposalV1ConvaiAgentsAgentIDMergeProposalsPost {
    ///                 source_branch_id: "source_branch_id".to_string(),
    ///                 target_branch_id: "target_branch_id".to_string(),
    ///                 title: "title".to_string(),
    ///                 description: None,
    ///                 requested_reviewer_user_ids: None,
    ///                 triage_ticket_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        agent_id: &str,
        request: &BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPost,
        options: Option<RequestOptions>,
    ) -> Result<CreateAgentMergeProposalResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agents/{}/merge-proposals", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a single merge_proposal.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `merge_proposal_id` - Unique identifier for the merge_proposal.
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
    ///         .merge_proposals
    ///         .get(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtmprop_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        agent_id: &str,
        merge_proposal_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AgentMergeProposalResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/agents/{}/merge-proposals/{}",
                    agent_id, merge_proposal_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Edit an open merge_proposal's title or description, or close it. The author closing it is recorded as withdrawn; anyone else as rejected.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `merge_proposal_id` - Unique identifier for the merge_proposal.
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
    ///         .merge_proposals
    ///         .update(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtmprop_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///             &BodyUpdateAMergeProposalV1ConvaiAgentsAgentIDMergeProposalsMergeProposalIDPatch {
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
        merge_proposal_id: &str,
        request: &BodyUpdateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdPatch,
        options: Option<RequestOptions>,
    ) -> Result<AgentMergeProposalResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/convai/agents/{}/merge-proposals/{}",
                    agent_id, merge_proposal_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Approve a merge_proposal or request changes on it. A user's latest review replaces their previous one. Non-admins need an approval from another user before the merge is allowed.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `merge_proposal_id` - Unique identifier for the merge_proposal.
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
    ///     client.agents.merge_proposals.submit_review(&"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(), &"agtmprop_8901k4t9z5defmb8vh3e9361y7nj".to_string(), &BodyReviewAMergeProposalV1ConvaiAgentsAgentIDMergeProposalsMergeProposalIDReviewsPost {
    ///         state: MergeProposalReviewState::Approved,
    ///         comment: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn submit_review(
        &self,
        agent_id: &str,
        merge_proposal_id: &str,
        request: &BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost,
        options: Option<RequestOptions>,
    ) -> Result<AgentMergeProposalResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/agents/{}/merge-proposals/{}/reviews",
                    agent_id, merge_proposal_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Leave a comment on a merge_proposal without recording a review verdict. Unlike reviews, comments accumulate and can still be added once the merge_proposal is merged or closed.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `merge_proposal_id` - Unique identifier for the merge_proposal.
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
    ///     client.agents.merge_proposals.add_comment(&"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(), &"agtmprop_8901k4t9z5defmb8vh3e9361y7nj".to_string(), &BodyCommentOnAMergeProposalV1ConvaiAgentsAgentIDMergeProposalsMergeProposalIDCommentsPost {
    ///         body: "body".to_string()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn add_comment(
        &self,
        agent_id: &str,
        merge_proposal_id: &str,
        request: &BodyCommentOnAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdCommentsPost,
        options: Option<RequestOptions>,
    ) -> Result<AgentMergeProposalResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/agents/{}/merge-proposals/{}/comments",
                    agent_id, merge_proposal_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Execute the merge. The caller must have write access to the target branch (admins only, for a protected branch), so this is where a reviewer approves and merges a request opened by someone who could not.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `merge_proposal_id` - Unique identifier for the merge_proposal.
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
    ///         .merge_proposals
    ///         .merge(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &"agtmprop_8901k4t9z5defmb8vh3e9361y7nj".to_string(),
    ///             &BodyMergeAMergeProposalV1ConvaiAgentsAgentIDMergeProposalsMergeProposalIDMergePost {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn merge(
        &self,
        agent_id: &str,
        merge_proposal_id: &str,
        request: &BodyMergeAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdMergePost,
        options: Option<RequestOptions>,
    ) -> Result<AgentMergeProposalResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/agents/{}/merge-proposals/{}/merge",
                    agent_id, merge_proposal_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
