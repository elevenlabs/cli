pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TicketMergeProposalLinkResponseModel {
    #[serde(default)]
    pub merge_proposal_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_by_user_id: Option<String>,
    #[serde(default)]
    pub linked_at_unix_secs: i64,
}

impl TicketMergeProposalLinkResponseModel {
    pub fn builder() -> TicketMergeProposalLinkResponseModelBuilder {
        <TicketMergeProposalLinkResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TicketMergeProposalLinkResponseModelBuilder {
    merge_proposal_id: Option<String>,
    linked_by_user_id: Option<String>,
    linked_at_unix_secs: Option<i64>,
}

impl TicketMergeProposalLinkResponseModelBuilder {
    pub fn merge_proposal_id(mut self, value: impl Into<String>) -> Self {
        self.merge_proposal_id = Some(value.into());
        self
    }

    pub fn linked_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.linked_by_user_id = Some(value.into());
        self
    }

    pub fn linked_at_unix_secs(mut self, value: i64) -> Self {
        self.linked_at_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TicketMergeProposalLinkResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`merge_proposal_id`](TicketMergeProposalLinkResponseModelBuilder::merge_proposal_id)
    /// - [`linked_at_unix_secs`](TicketMergeProposalLinkResponseModelBuilder::linked_at_unix_secs)
    pub fn build(self) -> Result<TicketMergeProposalLinkResponseModel, BuildError> {
        Ok(TicketMergeProposalLinkResponseModel {
            merge_proposal_id: self.merge_proposal_id.ok_or_else(|| BuildError::missing_field("merge_proposal_id"))?,
            linked_by_user_id: self.linked_by_user_id,
            linked_at_unix_secs: self.linked_at_unix_secs.ok_or_else(|| BuildError::missing_field("linked_at_unix_secs"))?,
        })
    }
}
