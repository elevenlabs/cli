pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Dependencies that use an auth connection
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthConnectionDependencies {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AuthConnectionDependenciesToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_servers: Option<Vec<AuthConnectionDependenciesMcpServersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_connections: Option<Vec<DependentIntegrationConnectionIdentifier>>,
}

impl AuthConnectionDependencies {
    pub fn builder() -> AuthConnectionDependenciesBuilder {
        <AuthConnectionDependenciesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthConnectionDependenciesBuilder {
    tools: Option<Vec<AuthConnectionDependenciesToolsItem>>,
    mcp_servers: Option<Vec<AuthConnectionDependenciesMcpServersItem>>,
    integration_connections: Option<Vec<DependentIntegrationConnectionIdentifier>>,
}

impl AuthConnectionDependenciesBuilder {
    pub fn tools(mut self, value: Vec<AuthConnectionDependenciesToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn mcp_servers(mut self, value: Vec<AuthConnectionDependenciesMcpServersItem>) -> Self {
        self.mcp_servers = Some(value);
        self
    }

    pub fn integration_connections(mut self, value: Vec<DependentIntegrationConnectionIdentifier>) -> Self {
        self.integration_connections = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthConnectionDependencies`].
    pub fn build(self) -> Result<AuthConnectionDependencies, BuildError> {
        Ok(AuthConnectionDependencies {
            tools: self.tools,
            mcp_servers: self.mcp_servers,
            integration_connections: self.integration_connections,
        })
    }
}
