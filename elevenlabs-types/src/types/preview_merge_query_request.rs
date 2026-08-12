pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for preview_merge
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewMergeQueryRequest {
    /// The ID of the target branch to merge into.
    #[serde(default)]
    pub target_branch_id: String,
    /// When true, source branch changes always win conflicts regardless of timestamps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl PreviewMergeQueryRequest {
    pub fn builder() -> PreviewMergeQueryRequestBuilder {
        <PreviewMergeQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewMergeQueryRequestBuilder {
    target_branch_id: Option<String>,
    force: Option<bool>,
}

impl PreviewMergeQueryRequestBuilder {
    pub fn target_branch_id(mut self, value: impl Into<String>) -> Self {
        self.target_branch_id = Some(value.into());
        self
    }

    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewMergeQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_branch_id`](PreviewMergeQueryRequestBuilder::target_branch_id)
    pub fn build(self) -> Result<PreviewMergeQueryRequest, BuildError> {
        Ok(PreviewMergeQueryRequest {
            target_branch_id: self.target_branch_id.ok_or_else(|| BuildError::missing_field("target_branch_id"))?,
            force: self.force,
        })
    }
}

