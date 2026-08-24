pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetToolExecutionsPageResponseModel {
    #[serde(default)]
    pub executions: Vec<ToolExecutionResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl GetToolExecutionsPageResponseModel {
    pub fn builder() -> GetToolExecutionsPageResponseModelBuilder {
        <GetToolExecutionsPageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetToolExecutionsPageResponseModelBuilder {
    executions: Option<Vec<ToolExecutionResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetToolExecutionsPageResponseModelBuilder {
    pub fn executions(mut self, value: Vec<ToolExecutionResponseModel>) -> Self {
        self.executions = Some(value);
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

    /// Consumes the builder and constructs a [`GetToolExecutionsPageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`executions`](GetToolExecutionsPageResponseModelBuilder::executions)
    /// - [`has_more`](GetToolExecutionsPageResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetToolExecutionsPageResponseModel, BuildError> {
        Ok(GetToolExecutionsPageResponseModel {
            executions: self.executions.ok_or_else(|| BuildError::missing_field("executions"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
