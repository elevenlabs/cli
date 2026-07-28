pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceGroupsSearchQueryRequest {
    /// Name of the target group.
    #[serde(default)]
    pub name: String,
}

impl WorkspaceGroupsSearchQueryRequest {
    pub fn builder() -> WorkspaceGroupsSearchQueryRequestBuilder {
        <WorkspaceGroupsSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceGroupsSearchQueryRequestBuilder {
    name: Option<String>,
}

impl WorkspaceGroupsSearchQueryRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceGroupsSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](WorkspaceGroupsSearchQueryRequestBuilder::name)
    pub fn build(self) -> Result<WorkspaceGroupsSearchQueryRequest, BuildError> {
        Ok(WorkspaceGroupsSearchQueryRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

