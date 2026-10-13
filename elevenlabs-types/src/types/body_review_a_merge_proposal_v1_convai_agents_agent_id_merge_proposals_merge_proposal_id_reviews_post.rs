pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost {
    /// The review verdict.
    pub state: MergeProposalReviewState,
    /// Optional comment to leave with the review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost {
    pub fn builder() -> BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPostBuilder {
        <BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPostBuilder {
    state: Option<MergeProposalReviewState>,
    comment: Option<String>,
}

impl BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPostBuilder {
    pub fn state(mut self, value: MergeProposalReviewState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPostBuilder::state)
    pub fn build(self) -> Result<BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost, BuildError> {
        Ok(BodyReviewAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdReviewsPost {
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            comment: self.comment,
        })
    }
}

