pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for a list of MCP Server configurations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpServersResponseModel {
    #[serde(default)]
    pub mcp_servers: Vec<McpServerResponseModel>,
}

impl McpServersResponseModel {
    pub fn builder() -> McpServersResponseModelBuilder {
        <McpServersResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServersResponseModelBuilder {
    mcp_servers: Option<Vec<McpServerResponseModel>>,
}

impl McpServersResponseModelBuilder {
    pub fn mcp_servers(mut self, value: Vec<McpServerResponseModel>) -> Self {
        self.mcp_servers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServersResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mcp_servers`](McpServersResponseModelBuilder::mcp_servers)
    pub fn build(self) -> Result<McpServersResponseModel, BuildError> {
        Ok(McpServersResponseModel {
            mcp_servers: self.mcp_servers.ok_or_else(|| BuildError::missing_field("mcp_servers"))?,
        })
    }
}
