pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FlowsTemplatesRunsListQueryRequest {
    /// Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many runs to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Only return runs of this template version id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl FlowsTemplatesRunsListQueryRequest {
    pub fn builder() -> FlowsTemplatesRunsListQueryRequestBuilder {
        <FlowsTemplatesRunsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FlowsTemplatesRunsListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    version_id: Option<String>,
}

impl FlowsTemplatesRunsListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FlowsTemplatesRunsListQueryRequest`].
    pub fn build(self) -> Result<FlowsTemplatesRunsListQueryRequest, BuildError> {
        Ok(FlowsTemplatesRunsListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            version_id: self.version_id,
        })
    }
}

