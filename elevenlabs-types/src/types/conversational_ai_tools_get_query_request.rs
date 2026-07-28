pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiToolsGetQueryRequest {
    /// Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl ConversationalAiToolsGetQueryRequest {
    pub fn builder() -> ConversationalAiToolsGetQueryRequestBuilder {
        <ConversationalAiToolsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiToolsGetQueryRequestBuilder {
    environment: Option<String>,
}

impl ConversationalAiToolsGetQueryRequestBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiToolsGetQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiToolsGetQueryRequest, BuildError> {
        Ok(ConversationalAiToolsGetQueryRequest {
            environment: self.environment,
        })
    }
}

