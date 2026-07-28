pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Custom SIP header for phone transfers with a dynamic variable reference.
/// The value is a variable name that will be resolved at runtime.
/// Value is not validated here since it will be substituted with actual value later.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomSipHeaderWithDynamicVariable {
    /// The SIP header name (e.g., 'X-Customer-ID')
    #[serde(default)]
    pub key: String,
    /// The dynamic variable name to resolve
    #[serde(default)]
    pub value: String,
}

impl CustomSipHeaderWithDynamicVariable {
    pub fn builder() -> CustomSipHeaderWithDynamicVariableBuilder {
        <CustomSipHeaderWithDynamicVariableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomSipHeaderWithDynamicVariableBuilder {
    key: Option<String>,
    value: Option<String>,
}

impl CustomSipHeaderWithDynamicVariableBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CustomSipHeaderWithDynamicVariable`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](CustomSipHeaderWithDynamicVariableBuilder::key)
    /// - [`value`](CustomSipHeaderWithDynamicVariableBuilder::value)
    pub fn build(self) -> Result<CustomSipHeaderWithDynamicVariable, BuildError> {
        Ok(CustomSipHeaderWithDynamicVariable {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
