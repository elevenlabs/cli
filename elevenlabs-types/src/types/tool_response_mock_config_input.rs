pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolResponseMockConfigInput {
    /// If the list is empty, the mock will always activate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_conditions: Option<Vec<UnitTestToolCallParameter>>,
    /// The return value the LLM sees when this mock is active.
    #[serde(default)]
    pub mock_result: String,
}

impl ToolResponseMockConfigInput {
    pub fn builder() -> ToolResponseMockConfigInputBuilder {
        <ToolResponseMockConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolResponseMockConfigInputBuilder {
    parameter_conditions: Option<Vec<UnitTestToolCallParameter>>,
    mock_result: Option<String>,
}

impl ToolResponseMockConfigInputBuilder {
    pub fn parameter_conditions(mut self, value: Vec<UnitTestToolCallParameter>) -> Self {
        self.parameter_conditions = Some(value);
        self
    }

    pub fn mock_result(mut self, value: impl Into<String>) -> Self {
        self.mock_result = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolResponseMockConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mock_result`](ToolResponseMockConfigInputBuilder::mock_result)
    pub fn build(self) -> Result<ToolResponseMockConfigInput, BuildError> {
        Ok(ToolResponseMockConfigInput {
            parameter_conditions: self.parameter_conditions,
            mock_result: self.mock_result.ok_or_else(|| BuildError::missing_field("mock_result"))?,
        })
    }
}
