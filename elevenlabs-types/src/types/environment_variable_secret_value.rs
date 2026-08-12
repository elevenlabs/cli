pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentVariableSecretValue {
    #[serde(default)]
    pub secret_id: String,
}

impl EnvironmentVariableSecretValue {
    pub fn builder() -> EnvironmentVariableSecretValueBuilder {
        <EnvironmentVariableSecretValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentVariableSecretValueBuilder {
    secret_id: Option<String>,
}

impl EnvironmentVariableSecretValueBuilder {
    pub fn secret_id(mut self, value: impl Into<String>) -> Self {
        self.secret_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentVariableSecretValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`secret_id`](EnvironmentVariableSecretValueBuilder::secret_id)
    pub fn build(self) -> Result<EnvironmentVariableSecretValue, BuildError> {
        Ok(EnvironmentVariableSecretValue {
            secret_id: self.secret_id.ok_or_else(|| BuildError::missing_field("secret_id"))?,
        })
    }
}
