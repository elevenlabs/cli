pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_agents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAgentsQueryRequest {
    /// Type of dependent agents to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_type: Option<KnowledgeBaseDependentType>,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetAgentsQueryRequest {
    pub fn builder() -> GetAgentsQueryRequestBuilder {
        <GetAgentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentsQueryRequestBuilder {
    dependent_type: Option<KnowledgeBaseDependentType>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl GetAgentsQueryRequestBuilder {
    pub fn dependent_type(mut self, value: KnowledgeBaseDependentType) -> Self {
        self.dependent_type = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAgentsQueryRequest`].
    pub fn build(self) -> Result<GetAgentsQueryRequest, BuildError> {
        Ok(GetAgentsQueryRequest {
            dependent_type: self.dependent_type,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

