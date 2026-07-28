pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiMcpServersToolsListQueryRequest {
    /// Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl ConversationalAiMcpServersToolsListQueryRequest {
    pub fn builder() -> ConversationalAiMcpServersToolsListQueryRequestBuilder {
        <ConversationalAiMcpServersToolsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiMcpServersToolsListQueryRequestBuilder {
    environment: Option<String>,
}

impl ConversationalAiMcpServersToolsListQueryRequestBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiMcpServersToolsListQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiMcpServersToolsListQueryRequest, BuildError> {
        Ok(ConversationalAiMcpServersToolsListQueryRequest {
            environment: self.environment,
        })
    }
}

