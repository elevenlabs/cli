pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// User-specific secret model that are not shared with other users in a workspace.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvAiUserSecretDbModel {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub encrypted_value: String,
    #[serde(default)]
    pub nonce: String,
    #[serde(default)]
    pub id: String,
}

impl ConvAiUserSecretDbModel {
    pub fn builder() -> ConvAiUserSecretDbModelBuilder {
        <ConvAiUserSecretDbModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvAiUserSecretDbModelBuilder {
    name: Option<String>,
    encrypted_value: Option<String>,
    nonce: Option<String>,
    id: Option<String>,
}

impl ConvAiUserSecretDbModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn encrypted_value(mut self, value: impl Into<String>) -> Self {
        self.encrypted_value = Some(value.into());
        self
    }

    pub fn nonce(mut self, value: impl Into<String>) -> Self {
        self.nonce = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvAiUserSecretDbModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConvAiUserSecretDbModelBuilder::name)
    /// - [`encrypted_value`](ConvAiUserSecretDbModelBuilder::encrypted_value)
    /// - [`nonce`](ConvAiUserSecretDbModelBuilder::nonce)
    /// - [`id`](ConvAiUserSecretDbModelBuilder::id)
    pub fn build(self) -> Result<ConvAiUserSecretDbModel, BuildError> {
        Ok(ConvAiUserSecretDbModel {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            encrypted_value: self.encrypted_value.ok_or_else(|| BuildError::missing_field("encrypted_value"))?,
            nonce: self.nonce.ok_or_else(|| BuildError::missing_field("nonce"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
