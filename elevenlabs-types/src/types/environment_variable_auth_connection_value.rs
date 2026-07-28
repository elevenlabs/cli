pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentVariableAuthConnectionValue {
    #[serde(default)]
    pub auth_connection_id: String,
}

impl EnvironmentVariableAuthConnectionValue {
    pub fn builder() -> EnvironmentVariableAuthConnectionValueBuilder {
        <EnvironmentVariableAuthConnectionValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVariableAuthConnectionValueBuilder {
    auth_connection_id: Option<String>,
}

impl EnvironmentVariableAuthConnectionValueBuilder {
    pub fn auth_connection_id(mut self, value: impl Into<String>) -> Self {
        self.auth_connection_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVariableAuthConnectionValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_connection_id`](EnvironmentVariableAuthConnectionValueBuilder::auth_connection_id)
    pub fn build(self) -> Result<EnvironmentVariableAuthConnectionValue, BuildError> {
        Ok(EnvironmentVariableAuthConnectionValue {
            auth_connection_id: self.auth_connection_id.ok_or_else(|| BuildError::missing_field("auth_connection_id"))?,
        })
    }
}
