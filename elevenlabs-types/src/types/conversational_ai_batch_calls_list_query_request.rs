pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiBatchCallsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_doc: Option<String>,
    /// Filter batch calls to a single agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl ConversationalAiBatchCallsListQueryRequest {
    pub fn builder() -> ConversationalAiBatchCallsListQueryRequestBuilder {
        <ConversationalAiBatchCallsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiBatchCallsListQueryRequestBuilder {
    limit: Option<i64>,
    last_doc: Option<String>,
    agent_id: Option<String>,
}

impl ConversationalAiBatchCallsListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn last_doc(mut self, value: impl Into<String>) -> Self {
        self.last_doc = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiBatchCallsListQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiBatchCallsListQueryRequest, BuildError> {
        Ok(ConversationalAiBatchCallsListQueryRequest {
            limit: self.limit,
            last_doc: self.last_doc,
            agent_id: self.agent_id,
        })
    }
}

