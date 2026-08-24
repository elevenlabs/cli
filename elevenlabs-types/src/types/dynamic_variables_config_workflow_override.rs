pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DynamicVariablesConfigWorkflowOverride {
    /// A dictionary of dynamic variable placeholders and their values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variable_placeholders: Option<HashMap<String, serde_json::Value>>,
}

impl DynamicVariablesConfigWorkflowOverride {
    pub fn builder() -> DynamicVariablesConfigWorkflowOverrideBuilder {
        <DynamicVariablesConfigWorkflowOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DynamicVariablesConfigWorkflowOverrideBuilder {
    dynamic_variable_placeholders: Option<HashMap<String, serde_json::Value>>,
}

impl DynamicVariablesConfigWorkflowOverrideBuilder {
    pub fn dynamic_variable_placeholders(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.dynamic_variable_placeholders = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DynamicVariablesConfigWorkflowOverride`].
    pub fn build(self) -> Result<DynamicVariablesConfigWorkflowOverride, BuildError> {
        Ok(DynamicVariablesConfigWorkflowOverride {
            dynamic_variable_placeholders: self.dynamic_variable_placeholders,
        })
    }
}
