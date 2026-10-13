pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPost {
    /// Branch whose changes should be merged.
    #[serde(default)]
    pub source_branch_id: String,
    /// Branch that should receive the changes.
    #[serde(default)]
    pub target_branch_id: String,
    /// Short title for the merge_proposal.
    #[serde(default)]
    pub title: String,
    /// Optional longer description for reviewers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User IDs to request a review from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_reviewer_user_ids: Option<Vec<String>>,
    /// Triage ticket of this agent that the merge_proposal resolves. Merging it resolves the ticket if still open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triage_ticket_id: Option<String>,
}

impl BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPost {
    pub fn builder() -> BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPostBuilder {
        <BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPostBuilder {
    source_branch_id: Option<String>,
    target_branch_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    requested_reviewer_user_ids: Option<Vec<String>>,
    triage_ticket_id: Option<String>,
}

impl BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPostBuilder {
    pub fn source_branch_id(mut self, value: impl Into<String>) -> Self {
        self.source_branch_id = Some(value.into());
        self
    }

    pub fn target_branch_id(mut self, value: impl Into<String>) -> Self {
        self.target_branch_id = Some(value.into());
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

    pub fn triage_ticket_id(mut self, value: impl Into<String>) -> Self {
        self.triage_ticket_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_branch_id`](BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPostBuilder::source_branch_id)
    /// - [`target_branch_id`](BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPostBuilder::target_branch_id)
    /// - [`title`](BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPostBuilder::title)
    pub fn build(self) -> Result<BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPost, BuildError> {
        Ok(BodyCreateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsPost {
            source_branch_id: self.source_branch_id.ok_or_else(|| BuildError::missing_field("source_branch_id"))?,
            target_branch_id: self.target_branch_id.ok_or_else(|| BuildError::missing_field("target_branch_id"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            description: self.description,
            requested_reviewer_user_ids: self.requested_reviewer_user_ids,
            triage_ticket_id: self.triage_ticket_id,
        })
    }
}

