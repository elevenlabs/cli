pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolsResponseModel {
    #[serde(default)]
    pub tools: Vec<ToolResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl ToolsResponseModel {
    pub fn builder() -> ToolsResponseModelBuilder {
        <ToolsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsResponseModelBuilder {
    tools: Option<Vec<ToolResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl ToolsResponseModelBuilder {
    pub fn tools(mut self, value: Vec<ToolResponseModel>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tools`](ToolsResponseModelBuilder::tools)
    /// - [`has_more`](ToolsResponseModelBuilder::has_more)
    pub fn build(self) -> Result<ToolsResponseModel, BuildError> {
        Ok(ToolsResponseModel {
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
