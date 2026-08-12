pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpServerRequestModel {
    /// Configuration details for the MCP Server.
    pub config: McpServerConfigInput,
}

impl McpServerRequestModel {
    pub fn builder() -> McpServerRequestModelBuilder {
        <McpServerRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerRequestModelBuilder {
    config: Option<McpServerConfigInput>,
}

impl McpServerRequestModelBuilder {
    pub fn config(mut self, value: McpServerConfigInput) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServerRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](McpServerRequestModelBuilder::config)
    pub fn build(self) -> Result<McpServerRequestModel, BuildError> {
        Ok(McpServerRequestModel {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}

