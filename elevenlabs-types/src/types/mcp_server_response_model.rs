pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model representing an MCP Server configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpServerResponseModel {
    #[serde(default)]
    pub id: String,
    pub config: McpServerConfigOutput,
    /// The access information of the MCP Server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_info: Option<ResourceAccessInfo>,
    /// List of agents that depend on this MCP Server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_agents: Option<Vec<McpServerResponseModelDependentAgentsItem>>,
    /// The metadata of the MCP Server
    #[serde(default)]
    pub metadata: McpServerMetadataResponseModel,
}

impl McpServerResponseModel {
    pub fn builder() -> McpServerResponseModelBuilder {
        <McpServerResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerResponseModelBuilder {
    id: Option<String>,
    config: Option<McpServerConfigOutput>,
    access_info: Option<ResourceAccessInfo>,
    dependent_agents: Option<Vec<McpServerResponseModelDependentAgentsItem>>,
    metadata: Option<McpServerMetadataResponseModel>,
}

impl McpServerResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn config(mut self, value: McpServerConfigOutput) -> Self {
        self.config = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    pub fn dependent_agents(mut self, value: Vec<McpServerResponseModelDependentAgentsItem>) -> Self {
        self.dependent_agents = Some(value);
        self
    }

    pub fn metadata(mut self, value: McpServerMetadataResponseModel) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServerResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](McpServerResponseModelBuilder::id)
    /// - [`config`](McpServerResponseModelBuilder::config)
    /// - [`metadata`](McpServerResponseModelBuilder::metadata)
    pub fn build(self) -> Result<McpServerResponseModel, BuildError> {
        Ok(McpServerResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
            access_info: self.access_info,
            dependent_agents: self.dependent_agents,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
        })
    }
}
