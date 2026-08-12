pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsConversationsTopicsGetQueryRequest {
    /// Start of the window to view topics for. When set with to_unix_secs, per-day topics in the range are aggregated together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_unix_secs: Option<i64>,
    /// End of the window to view topics for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_unix_secs: Option<i64>,
}

impl AgentsConversationsTopicsGetQueryRequest {
    pub fn builder() -> AgentsConversationsTopicsGetQueryRequestBuilder {
        <AgentsConversationsTopicsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsConversationsTopicsGetQueryRequestBuilder {
    from_unix_secs: Option<i64>,
    to_unix_secs: Option<i64>,
}

impl AgentsConversationsTopicsGetQueryRequestBuilder {
    pub fn from_unix_secs(mut self, value: i64) -> Self {
        self.from_unix_secs = Some(value);
        self
    }

    pub fn to_unix_secs(mut self, value: i64) -> Self {
        self.to_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsConversationsTopicsGetQueryRequest`].
    pub fn build(self) -> Result<AgentsConversationsTopicsGetQueryRequest, BuildError> {
        Ok(AgentsConversationsTopicsGetQueryRequest {
            from_unix_secs: self.from_unix_secs,
            to_unix_secs: self.to_unix_secs,
        })
    }
}

