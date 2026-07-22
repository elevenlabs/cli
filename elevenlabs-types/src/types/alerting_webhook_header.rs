pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlertingWebhookHeader {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_secret: Option<bool>,
    /// Header value. May be null only for a secret header on update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl AlertingWebhookHeader {
    pub fn builder() -> AlertingWebhookHeaderBuilder {
        <AlertingWebhookHeaderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingWebhookHeaderBuilder {
    name: Option<String>,
    is_secret: Option<bool>,
    value: Option<String>,
}

impl AlertingWebhookHeaderBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_secret(mut self, value: bool) -> Self {
        self.is_secret = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AlertingWebhookHeader`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AlertingWebhookHeaderBuilder::name)
    pub fn build(self) -> Result<AlertingWebhookHeader, BuildError> {
        Ok(AlertingWebhookHeader {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_secret: self.is_secret,
            value: self.value,
        })
    }
}
