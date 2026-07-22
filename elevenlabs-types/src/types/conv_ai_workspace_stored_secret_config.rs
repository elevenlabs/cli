pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvAiWorkspaceStoredSecretConfig {
    pub r#type: String,
    #[serde(default)]
    pub secret_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub used_by: ConvAiStoredSecretDependencies,
}

impl ConvAiWorkspaceStoredSecretConfig {
    pub fn builder() -> ConvAiWorkspaceStoredSecretConfigBuilder {
        <ConvAiWorkspaceStoredSecretConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvAiWorkspaceStoredSecretConfigBuilder {
    r#type: Option<String>,
    secret_id: Option<String>,
    name: Option<String>,
    used_by: Option<ConvAiStoredSecretDependencies>,
}

impl ConvAiWorkspaceStoredSecretConfigBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn secret_id(mut self, value: impl Into<String>) -> Self {
        self.secret_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn used_by(mut self, value: ConvAiStoredSecretDependencies) -> Self {
        self.used_by = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConvAiWorkspaceStoredSecretConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ConvAiWorkspaceStoredSecretConfigBuilder::r#type)
    /// - [`secret_id`](ConvAiWorkspaceStoredSecretConfigBuilder::secret_id)
    /// - [`name`](ConvAiWorkspaceStoredSecretConfigBuilder::name)
    /// - [`used_by`](ConvAiWorkspaceStoredSecretConfigBuilder::used_by)
    pub fn build(self) -> Result<ConvAiWorkspaceStoredSecretConfig, BuildError> {
        Ok(ConvAiWorkspaceStoredSecretConfig {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            secret_id: self.secret_id.ok_or_else(|| BuildError::missing_field("secret_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            used_by: self.used_by.ok_or_else(|| BuildError::missing_field("used_by"))?,
        })
    }
}
