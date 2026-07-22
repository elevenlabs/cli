pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// References an environment variable of type 'auth_connection' by label.
/// At runtime, resolves to the auth connection for the current environment,
/// falling back to the default environment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct EnvironmentAuthConnectionLocator {
    pub env_var_label: String,
}

impl EnvironmentAuthConnectionLocator {
    pub fn builder() -> EnvironmentAuthConnectionLocatorBuilder {
        <EnvironmentAuthConnectionLocatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentAuthConnectionLocatorBuilder {
    env_var_label: Option<String>,
}

impl EnvironmentAuthConnectionLocatorBuilder {
    pub fn env_var_label(mut self, value: impl Into<String>) -> Self {
        self.env_var_label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentAuthConnectionLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`env_var_label`](EnvironmentAuthConnectionLocatorBuilder::env_var_label)
    pub fn build(self) -> Result<EnvironmentAuthConnectionLocator, BuildError> {
        Ok(EnvironmentAuthConnectionLocator {
            env_var_label: self.env_var_label.ok_or_else(|| BuildError::missing_field("env_var_label"))?,
        })
    }
}
