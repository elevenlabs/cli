pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Configuration for extracting values from tool responses and assigning them to dynamic variables.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DynamicVariableAssignment {
    /// The source to extract the value from. Currently only 'response' is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The name of the dynamic variable to assign the extracted value to
    #[serde(default)]
    pub dynamic_variable: String,
    /// Dot notation path to extract the value from the source (e.g., 'user.name' or 'data.0.id')
    #[serde(default)]
    pub value_path: String,
    /// If true, this assignment's value will be removed from the tool response before sending to the LLM and transcript, but still processed for variable assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sanitize: Option<bool>,
    /// If true, non-scalar values (lists, objects) extracted from the tool response are stored as their native type instead of being stringified to JSON. Enable this to use extracted arrays directly as list dynamic variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_native_type: Option<bool>,
}

impl DynamicVariableAssignment {
    pub fn builder() -> DynamicVariableAssignmentBuilder {
        <DynamicVariableAssignmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DynamicVariableAssignmentBuilder {
    source: Option<String>,
    dynamic_variable: Option<String>,
    value_path: Option<String>,
    sanitize: Option<bool>,
    preserve_native_type: Option<bool>,
}

impl DynamicVariableAssignmentBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn dynamic_variable(mut self, value: impl Into<String>) -> Self {
        self.dynamic_variable = Some(value.into());
        self
    }

    pub fn value_path(mut self, value: impl Into<String>) -> Self {
        self.value_path = Some(value.into());
        self
    }

    pub fn sanitize(mut self, value: bool) -> Self {
        self.sanitize = Some(value);
        self
    }

    pub fn preserve_native_type(mut self, value: bool) -> Self {
        self.preserve_native_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DynamicVariableAssignment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dynamic_variable`](DynamicVariableAssignmentBuilder::dynamic_variable)
    /// - [`value_path`](DynamicVariableAssignmentBuilder::value_path)
    pub fn build(self) -> Result<DynamicVariableAssignment, BuildError> {
        Ok(DynamicVariableAssignment {
            source: self.source,
            dynamic_variable: self.dynamic_variable.ok_or_else(|| BuildError::missing_field("dynamic_variable"))?,
            value_path: self.value_path.ok_or_else(|| BuildError::missing_field("value_path"))?,
            sanitize: self.sanitize,
            preserve_native_type: self.preserve_native_type,
        })
    }
}
