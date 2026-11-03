pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsAnalysisItemsDependentAgentsGetQueryRequest {
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsAnalysisItemsDependentAgentsGetQueryRequest {
    pub fn builder() -> AgentsAnalysisItemsDependentAgentsGetQueryRequestBuilder {
        <AgentsAnalysisItemsDependentAgentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsAnalysisItemsDependentAgentsGetQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl AgentsAnalysisItemsDependentAgentsGetQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsAnalysisItemsDependentAgentsGetQueryRequest`].
    pub fn build(self) -> Result<AgentsAnalysisItemsDependentAgentsGetQueryRequest, BuildError> {
        Ok(AgentsAnalysisItemsDependentAgentsGetQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

