pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolResponseModel {
    #[serde(default)]
    pub id: String,
    /// The type of tool
    pub tool_config: ToolResponseModelToolConfig,
    pub access_info: ResourceAccessInfo,
    #[serde(default)]
    pub usage_stats: ToolUsageStatsResponseModel,
    /// Mock responses with optional parameter conditions. Evaluated top-to-bottom; first match wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mocks: Option<Vec<ToolResponseMockConfigOutput>>,
}

impl ToolResponseModel {
    pub fn builder() -> ToolResponseModelBuilder {
        <ToolResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolResponseModelBuilder {
    id: Option<String>,
    tool_config: Option<ToolResponseModelToolConfig>,
    access_info: Option<ResourceAccessInfo>,
    usage_stats: Option<ToolUsageStatsResponseModel>,
    response_mocks: Option<Vec<ToolResponseMockConfigOutput>>,
}

impl ToolResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tool_config(mut self, value: ToolResponseModelToolConfig) -> Self {
        self.tool_config = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    pub fn usage_stats(mut self, value: ToolUsageStatsResponseModel) -> Self {
        self.usage_stats = Some(value);
        self
    }

    pub fn response_mocks(mut self, value: Vec<ToolResponseMockConfigOutput>) -> Self {
        self.response_mocks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ToolResponseModelBuilder::id)
    /// - [`tool_config`](ToolResponseModelBuilder::tool_config)
    /// - [`access_info`](ToolResponseModelBuilder::access_info)
    /// - [`usage_stats`](ToolResponseModelBuilder::usage_stats)
    pub fn build(self) -> Result<ToolResponseModel, BuildError> {
        Ok(ToolResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tool_config: self.tool_config.ok_or_else(|| BuildError::missing_field("tool_config"))?,
            access_info: self.access_info.ok_or_else(|| BuildError::missing_field("access_info"))?,
            usage_stats: self.usage_stats.ok_or_else(|| BuildError::missing_field("usage_stats"))?,
            response_mocks: self.response_mocks,
        })
    }
}
