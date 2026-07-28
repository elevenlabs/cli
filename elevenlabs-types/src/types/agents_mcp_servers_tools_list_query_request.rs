pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsMcpServersToolsListQueryRequest {
    /// Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl AgentsMcpServersToolsListQueryRequest {
    pub fn builder() -> AgentsMcpServersToolsListQueryRequestBuilder {
        <AgentsMcpServersToolsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsMcpServersToolsListQueryRequestBuilder {
    environment: Option<String>,
}

impl AgentsMcpServersToolsListQueryRequestBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsMcpServersToolsListQueryRequest`].
    pub fn build(self) -> Result<AgentsMcpServersToolsListQueryRequest, BuildError> {
        Ok(AgentsMcpServersToolsListQueryRequest {
            environment: self.environment,
        })
    }
}

