pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MergeProposalReview {
    #[serde(default)]
    pub user_id: String,
    pub state: MergeProposalReviewState,
    #[serde(default)]
    pub submitted_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_role: Option<MergeProposalReviewReviewerRole>,
}

impl MergeProposalReview {
    pub fn builder() -> MergeProposalReviewBuilder {
        <MergeProposalReviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeProposalReviewBuilder {
    user_id: Option<String>,
    state: Option<MergeProposalReviewState>,
    submitted_at: Option<i64>,
    comment: Option<String>,
    reviewer_role: Option<MergeProposalReviewReviewerRole>,
}

impl MergeProposalReviewBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn state(mut self, value: MergeProposalReviewState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn submitted_at(mut self, value: i64) -> Self {
        self.submitted_at = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    pub fn reviewer_role(mut self, value: MergeProposalReviewReviewerRole) -> Self {
        self.reviewer_role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergeProposalReview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](MergeProposalReviewBuilder::user_id)
    /// - [`state`](MergeProposalReviewBuilder::state)
    /// - [`submitted_at`](MergeProposalReviewBuilder::submitted_at)
    pub fn build(self) -> Result<MergeProposalReview, BuildError> {
        Ok(MergeProposalReview {
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            submitted_at: self.submitted_at.ok_or_else(|| BuildError::missing_field("submitted_at"))?,
            comment: self.comment,
            reviewer_role: self.reviewer_role,
        })
    }
}
