pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResellerWorkspacesResponse {
    #[serde(default)]
    pub workspaces: Vec<ResellerWorkspaceSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ResellerWorkspacesResponse {
    pub fn builder() -> ResellerWorkspacesResponseBuilder {
        <ResellerWorkspacesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResellerWorkspacesResponseBuilder {
    workspaces: Option<Vec<ResellerWorkspaceSummary>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl ResellerWorkspacesResponseBuilder {
    pub fn workspaces(mut self, value: Vec<ResellerWorkspaceSummary>) -> Self {
        self.workspaces = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResellerWorkspacesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspaces`](ResellerWorkspacesResponseBuilder::workspaces)
    pub fn build(self) -> Result<ResellerWorkspacesResponse, BuildError> {
        Ok(ResellerWorkspacesResponse {
            workspaces: self.workspaces.ok_or_else(|| BuildError::missing_field("workspaces"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
