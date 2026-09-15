pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A custom webhook header value, encrypted at rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlertingWebhookSecretHeaderValue {
    #[serde(default)]
    pub encrypted_value: String,
    #[serde(default)]
    pub nonce: String,
}

impl AlertingWebhookSecretHeaderValue {
    pub fn builder() -> AlertingWebhookSecretHeaderValueBuilder {
        <AlertingWebhookSecretHeaderValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingWebhookSecretHeaderValueBuilder {
    encrypted_value: Option<String>,
    nonce: Option<String>,
}

impl AlertingWebhookSecretHeaderValueBuilder {
    pub fn encrypted_value(mut self, value: impl Into<String>) -> Self {
        self.encrypted_value = Some(value.into());
        self
    }

    pub fn nonce(mut self, value: impl Into<String>) -> Self {
        self.nonce = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AlertingWebhookSecretHeaderValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`encrypted_value`](AlertingWebhookSecretHeaderValueBuilder::encrypted_value)
    /// - [`nonce`](AlertingWebhookSecretHeaderValueBuilder::nonce)
    pub fn build(self) -> Result<AlertingWebhookSecretHeaderValue, BuildError> {
        Ok(AlertingWebhookSecretHeaderValue {
            encrypted_value: self.encrypted_value.ok_or_else(|| BuildError::missing_field("encrypted_value"))?,
            nonce: self.nonce.ok_or_else(|| BuildError::missing_field("nonce"))?,
        })
    }
}
