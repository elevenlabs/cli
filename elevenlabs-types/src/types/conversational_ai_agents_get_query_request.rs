pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiAgentsGetQueryRequest {
    /// The ID of the agent version to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// The ID of the branch to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
}

impl ConversationalAiAgentsGetQueryRequest {
    pub fn builder() -> ConversationalAiAgentsGetQueryRequestBuilder {
        <ConversationalAiAgentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiAgentsGetQueryRequestBuilder {
    version_id: Option<String>,
    branch_id: Option<String>,
}

impl ConversationalAiAgentsGetQueryRequestBuilder {
    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiAgentsGetQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiAgentsGetQueryRequest, BuildError> {
        Ok(ConversationalAiAgentsGetQueryRequest {
            version_id: self.version_id,
            branch_id: self.branch_id,
        })
    }
}

