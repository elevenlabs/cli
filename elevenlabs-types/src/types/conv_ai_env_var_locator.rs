pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Used to reference an environment variable by label.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ConvAiEnvVarLocator {
    pub env_var_label: String,
}

impl ConvAiEnvVarLocator {
    pub fn builder() -> ConvAiEnvVarLocatorBuilder {
        <ConvAiEnvVarLocatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvAiEnvVarLocatorBuilder {
    env_var_label: Option<String>,
}

impl ConvAiEnvVarLocatorBuilder {
    pub fn env_var_label(mut self, value: impl Into<String>) -> Self {
        self.env_var_label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvAiEnvVarLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`env_var_label`](ConvAiEnvVarLocatorBuilder::env_var_label)
    pub fn build(self) -> Result<ConvAiEnvVarLocator, BuildError> {
        Ok(ConvAiEnvVarLocator {
            env_var_label: self.env_var_label.ok_or_else(|| BuildError::missing_field("env_var_label"))?,
        })
    }
}
