pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiAgentsBranchesListQueryRequest {
    /// Whether archived branches should be included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    /// How many results at most should be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ConversationalAiAgentsBranchesListQueryRequest {
    pub fn builder() -> ConversationalAiAgentsBranchesListQueryRequestBuilder {
        <ConversationalAiAgentsBranchesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiAgentsBranchesListQueryRequestBuilder {
    include_archived: Option<bool>,
    limit: Option<i64>,
}

impl ConversationalAiAgentsBranchesListQueryRequestBuilder {
    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiAgentsBranchesListQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiAgentsBranchesListQueryRequest, BuildError> {
        Ok(ConversationalAiAgentsBranchesListQueryRequest {
            include_archived: self.include_archived,
            limit: self.limit,
        })
    }
}

