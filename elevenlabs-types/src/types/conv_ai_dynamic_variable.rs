pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Used to reference a dynamic variable.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ConvAiDynamicVariable {
    pub variable_name: String,
}

impl ConvAiDynamicVariable {
    pub fn builder() -> ConvAiDynamicVariableBuilder {
        <ConvAiDynamicVariableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvAiDynamicVariableBuilder {
    variable_name: Option<String>,
}

impl ConvAiDynamicVariableBuilder {
    pub fn variable_name(mut self, value: impl Into<String>) -> Self {
        self.variable_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvAiDynamicVariable`].
    /// This method will fail if any of the following fields are not set:
    /// - [`variable_name`](ConvAiDynamicVariableBuilder::variable_name)
    pub fn build(self) -> Result<ConvAiDynamicVariable, BuildError> {
        Ok(ConvAiDynamicVariable {
            variable_name: self.variable_name.ok_or_else(|| BuildError::missing_field("variable_name"))?,
        })
    }
}
