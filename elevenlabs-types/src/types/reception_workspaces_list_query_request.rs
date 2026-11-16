pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionWorkspacesListQueryRequest {
    /// Pagination cursor from previous response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Number of workspaces per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl ReceptionWorkspacesListQueryRequest {
    pub fn builder() -> ReceptionWorkspacesListQueryRequestBuilder {
        <ReceptionWorkspacesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionWorkspacesListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
}

impl ReceptionWorkspacesListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionWorkspacesListQueryRequest`].
    pub fn build(self) -> Result<ReceptionWorkspacesListQueryRequest, BuildError> {
        Ok(ReceptionWorkspacesListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
        })
    }
}

