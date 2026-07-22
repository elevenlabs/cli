pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolRequestModel {
    /// Configuration for the tool
    pub tool_config: ToolRequestModelToolConfig,
    /// Mock responses with optional parameter conditions. Evaluated top-to-bottom; first match wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mocks: Option<Vec<ToolResponseMockConfigInput>>,
}

impl ToolRequestModel {
    pub fn builder() -> ToolRequestModelBuilder {
        <ToolRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolRequestModelBuilder {
    tool_config: Option<ToolRequestModelToolConfig>,
    response_mocks: Option<Vec<ToolResponseMockConfigInput>>,
}

impl ToolRequestModelBuilder {
    pub fn tool_config(mut self, value: ToolRequestModelToolConfig) -> Self {
        self.tool_config = Some(value);
        self
    }

    pub fn response_mocks(mut self, value: Vec<ToolResponseMockConfigInput>) -> Self {
        self.response_mocks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_config`](ToolRequestModelBuilder::tool_config)
    pub fn build(self) -> Result<ToolRequestModel, BuildError> {
        Ok(ToolRequestModel {
            tool_config: self.tool_config.ok_or_else(|| BuildError::missing_field("tool_config"))?,
            response_mocks: self.response_mocks,
        })
    }
}
