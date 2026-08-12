pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct EnvironmentVariableAuthConnectionValueRequest {
    pub auth_connection_id: String,
}

impl EnvironmentVariableAuthConnectionValueRequest {
    pub fn builder() -> EnvironmentVariableAuthConnectionValueRequestBuilder {
        <EnvironmentVariableAuthConnectionValueRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVariableAuthConnectionValueRequestBuilder {
    auth_connection_id: Option<String>,
}

impl EnvironmentVariableAuthConnectionValueRequestBuilder {
    pub fn auth_connection_id(mut self, value: impl Into<String>) -> Self {
        self.auth_connection_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVariableAuthConnectionValueRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_connection_id`](EnvironmentVariableAuthConnectionValueRequestBuilder::auth_connection_id)
    pub fn build(self) -> Result<EnvironmentVariableAuthConnectionValueRequest, BuildError> {
        Ok(EnvironmentVariableAuthConnectionValueRequest {
            auth_connection_id: self.auth_connection_id.ok_or_else(|| BuildError::missing_field("auth_connection_id"))?,
        })
    }
}
