pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergeProposalComment {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub created_at: i64,
}

impl MergeProposalComment {
    pub fn builder() -> MergeProposalCommentBuilder {
        <MergeProposalCommentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeProposalCommentBuilder {
    id: Option<String>,
    user_id: Option<String>,
    body: Option<String>,
    created_at: Option<i64>,
}

impl MergeProposalCommentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergeProposalComment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MergeProposalCommentBuilder::id)
    /// - [`user_id`](MergeProposalCommentBuilder::user_id)
    /// - [`body`](MergeProposalCommentBuilder::body)
    /// - [`created_at`](MergeProposalCommentBuilder::created_at)
    pub fn build(self) -> Result<MergeProposalComment, BuildError> {
        Ok(MergeProposalComment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
