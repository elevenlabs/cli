pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct EnvironmentVariableSecretValueRequest {
    pub secret_id: String,
}

impl EnvironmentVariableSecretValueRequest {
    pub fn builder() -> EnvironmentVariableSecretValueRequestBuilder {
        <EnvironmentVariableSecretValueRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVariableSecretValueRequestBuilder {
    secret_id: Option<String>,
}

impl EnvironmentVariableSecretValueRequestBuilder {
    pub fn secret_id(mut self, value: impl Into<String>) -> Self {
        self.secret_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVariableSecretValueRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`secret_id`](EnvironmentVariableSecretValueRequestBuilder::secret_id)
    pub fn build(self) -> Result<EnvironmentVariableSecretValueRequest, BuildError> {
        Ok(EnvironmentVariableSecretValueRequest {
            secret_id: self.secret_id.ok_or_else(|| BuildError::missing_field("secret_id"))?,
        })
    }
}
