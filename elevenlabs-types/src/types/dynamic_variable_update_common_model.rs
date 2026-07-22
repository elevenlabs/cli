pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Tracks a dynamic variable update that occurred during tool execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DynamicVariableUpdateCommonModel {
    #[serde(default)]
    pub variable_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(default)]
    pub new_value: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub tool_request_id: String,
}

impl DynamicVariableUpdateCommonModel {
    pub fn builder() -> DynamicVariableUpdateCommonModelBuilder {
        <DynamicVariableUpdateCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DynamicVariableUpdateCommonModelBuilder {
    variable_name: Option<String>,
    old_value: Option<String>,
    new_value: Option<String>,
    updated_at: Option<f64>,
    tool_name: Option<String>,
    tool_request_id: Option<String>,
}

impl DynamicVariableUpdateCommonModelBuilder {
    pub fn variable_name(mut self, value: impl Into<String>) -> Self {
        self.variable_name = Some(value.into());
        self
    }

    pub fn old_value(mut self, value: impl Into<String>) -> Self {
        self.old_value = Some(value.into());
        self
    }

    pub fn new_value(mut self, value: impl Into<String>) -> Self {
        self.new_value = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: f64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn tool_request_id(mut self, value: impl Into<String>) -> Self {
        self.tool_request_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DynamicVariableUpdateCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`variable_name`](DynamicVariableUpdateCommonModelBuilder::variable_name)
    /// - [`new_value`](DynamicVariableUpdateCommonModelBuilder::new_value)
    /// - [`updated_at`](DynamicVariableUpdateCommonModelBuilder::updated_at)
    /// - [`tool_name`](DynamicVariableUpdateCommonModelBuilder::tool_name)
    /// - [`tool_request_id`](DynamicVariableUpdateCommonModelBuilder::tool_request_id)
    pub fn build(self) -> Result<DynamicVariableUpdateCommonModel, BuildError> {
        Ok(DynamicVariableUpdateCommonModel {
            variable_name: self.variable_name.ok_or_else(|| BuildError::missing_field("variable_name"))?,
            old_value: self.old_value,
            new_value: self.new_value.ok_or_else(|| BuildError::missing_field("new_value"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            tool_request_id: self.tool_request_id.ok_or_else(|| BuildError::missing_field("tool_request_id"))?,
        })
    }
}
