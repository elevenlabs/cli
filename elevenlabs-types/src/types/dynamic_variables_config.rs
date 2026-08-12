pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DynamicVariablesConfig {
    /// A dictionary of dynamic variable placeholders and their values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variable_placeholders: Option<HashMap<String, serde_json::Value>>,
}

impl DynamicVariablesConfig {
    pub fn builder() -> DynamicVariablesConfigBuilder {
        <DynamicVariablesConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DynamicVariablesConfigBuilder {
    dynamic_variable_placeholders: Option<HashMap<String, serde_json::Value>>,
}

impl DynamicVariablesConfigBuilder {
    pub fn dynamic_variable_placeholders(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.dynamic_variable_placeholders = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DynamicVariablesConfig`].
    pub fn build(self) -> Result<DynamicVariablesConfig, BuildError> {
        Ok(DynamicVariablesConfig {
            dynamic_variable_placeholders: self.dynamic_variable_placeholders,
        })
    }
}
