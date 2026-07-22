pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DynamicVariableSchemaOverride {
    /// The name of the dynamic variable to use
    #[serde(default)]
    pub dynamic_variable: String,
}

impl DynamicVariableSchemaOverride {
    pub fn builder() -> DynamicVariableSchemaOverrideBuilder {
        <DynamicVariableSchemaOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DynamicVariableSchemaOverrideBuilder {
    dynamic_variable: Option<String>,
}

impl DynamicVariableSchemaOverrideBuilder {
    pub fn dynamic_variable(mut self, value: impl Into<String>) -> Self {
        self.dynamic_variable = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DynamicVariableSchemaOverride`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dynamic_variable`](DynamicVariableSchemaOverrideBuilder::dynamic_variable)
    pub fn build(self) -> Result<DynamicVariableSchemaOverride, BuildError> {
        Ok(DynamicVariableSchemaOverride {
            dynamic_variable: self.dynamic_variable.ok_or_else(|| BuildError::missing_field("dynamic_variable"))?,
        })
    }
}
