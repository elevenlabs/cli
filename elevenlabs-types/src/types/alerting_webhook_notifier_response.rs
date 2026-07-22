pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AlertingWebhookNotifierResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub webhook_id: String,
}

impl AlertingWebhookNotifierResponse {
    pub fn builder() -> AlertingWebhookNotifierResponseBuilder {
        <AlertingWebhookNotifierResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingWebhookNotifierResponseBuilder {
    r#type: Option<String>,
    webhook_id: Option<String>,
}

impl AlertingWebhookNotifierResponseBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn webhook_id(mut self, value: impl Into<String>) -> Self {
        self.webhook_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AlertingWebhookNotifierResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhook_id`](AlertingWebhookNotifierResponseBuilder::webhook_id)
    pub fn build(self) -> Result<AlertingWebhookNotifierResponse, BuildError> {
        Ok(AlertingWebhookNotifierResponse {
            r#type: self.r#type,
            webhook_id: self.webhook_id.ok_or_else(|| BuildError::missing_field("webhook_id"))?,
        })
    }
}
