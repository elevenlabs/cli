pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentMergeProposalResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub source_branch_id: String,
    #[serde(default)]
    pub target_branch_id: String,
    #[serde(default)]
    pub source_tip_version_id_at_creation: String,
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_reviewer_user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<Vec<MergeProposalReview>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<MergeProposalComment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<AgentMergeProposalResponseOutcome>,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
    /// User ID of the merge_proposal's author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_user_id: Option<String>,
    /// Access information for the merge_proposal, including creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_info: Option<ResourceAccessInfo>,
}

impl AgentMergeProposalResponse {
    pub fn builder() -> AgentMergeProposalResponseBuilder {
        <AgentMergeProposalResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentMergeProposalResponseBuilder {
    id: Option<String>,
    agent_id: Option<String>,
    source_branch_id: Option<String>,
    target_branch_id: Option<String>,
    source_tip_version_id_at_creation: Option<String>,
    title: Option<String>,
    description: Option<String>,
    requested_reviewer_user_ids: Option<Vec<String>>,
    reviews: Option<Vec<MergeProposalReview>>,
    comments: Option<Vec<MergeProposalComment>>,
    outcome: Option<AgentMergeProposalResponseOutcome>,
    created_at: Option<i64>,
    updated_at: Option<i64>,
    author_user_id: Option<String>,
    access_info: Option<ResourceAccessInfo>,
}

impl AgentMergeProposalResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn source_branch_id(mut self, value: impl Into<String>) -> Self {
        self.source_branch_id = Some(value.into());
        self
    }

    pub fn target_branch_id(mut self, value: impl Into<String>) -> Self {
        self.target_branch_id = Some(value.into());
        self
    }

    pub fn source_tip_version_id_at_creation(mut self, value: impl Into<String>) -> Self {
        self.source_tip_version_id_at_creation = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn requested_reviewer_user_ids(mut self, value: Vec<String>) -> Self {
        self.requested_reviewer_user_ids = Some(value);
        self
    }

    pub fn reviews(mut self, value: Vec<MergeProposalReview>) -> Self {
        self.reviews = Some(value);
        self
    }

    pub fn comments(mut self, value: Vec<MergeProposalComment>) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn outcome(mut self, value: AgentMergeProposalResponseOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn author_user_id(mut self, value: impl Into<String>) -> Self {
        self.author_user_id = Some(value.into());
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentMergeProposalResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentMergeProposalResponseBuilder::id)
    /// - [`agent_id`](AgentMergeProposalResponseBuilder::agent_id)
    /// - [`source_branch_id`](AgentMergeProposalResponseBuilder::source_branch_id)
    /// - [`target_branch_id`](AgentMergeProposalResponseBuilder::target_branch_id)
    /// - [`source_tip_version_id_at_creation`](AgentMergeProposalResponseBuilder::source_tip_version_id_at_creation)
    /// - [`title`](AgentMergeProposalResponseBuilder::title)
    /// - [`created_at`](AgentMergeProposalResponseBuilder::created_at)
    /// - [`updated_at`](AgentMergeProposalResponseBuilder::updated_at)
    pub fn build(self) -> Result<AgentMergeProposalResponse, BuildError> {
        Ok(AgentMergeProposalResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            source_branch_id: self.source_branch_id.ok_or_else(|| BuildError::missing_field("source_branch_id"))?,
            target_branch_id: self.target_branch_id.ok_or_else(|| BuildError::missing_field("target_branch_id"))?,
            source_tip_version_id_at_creation: self.source_tip_version_id_at_creation.ok_or_else(|| BuildError::missing_field("source_tip_version_id_at_creation"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description,
            requested_reviewer_user_ids: self.requested_reviewer_user_ids,
            reviews: self.reviews,
            comments: self.comments,
            outcome: self.outcome,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            author_user_id: self.author_user_id,
            access_info: self.access_info,
        })
    }
}
