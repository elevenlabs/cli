pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolMockConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_is_error: Option<bool>,
}

impl ToolMockConfig {
    pub fn builder() -> ToolMockConfigBuilder {
        <ToolMockConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolMockConfigBuilder {
    default_return_value: Option<String>,
    default_is_error: Option<bool>,
}

impl ToolMockConfigBuilder {
    pub fn default_return_value(mut self, value: impl Into<String>) -> Self {
        self.default_return_value = Some(value.into());
        self
    }

    pub fn default_is_error(mut self, value: bool) -> Self {
        self.default_is_error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolMockConfig`].
    pub fn build(self) -> Result<ToolMockConfig, BuildError> {
        Ok(ToolMockConfig {
            default_return_value: self.default_return_value,
            default_is_error: self.default_is_error,
        })
    }
}
