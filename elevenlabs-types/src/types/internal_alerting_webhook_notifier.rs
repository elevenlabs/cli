pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InternalAlertingWebhookNotifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The URL to send alert lifecycle notifications to.
    #[serde(default)]
    pub url: String,
    /// HTTP method used when calling the webhook URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<AlertingWebhookMethod>,
    /// Custom request headers sent with every notification. Secret header values are encrypted at rest and never returned by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<AlertingWebhookHeader>>,
}

impl InternalAlertingWebhookNotifier {
    pub fn builder() -> InternalAlertingWebhookNotifierBuilder {
        <InternalAlertingWebhookNotifierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InternalAlertingWebhookNotifierBuilder {
    r#type: Option<String>,
    url: Option<String>,
    method: Option<AlertingWebhookMethod>,
    headers: Option<Vec<AlertingWebhookHeader>>,
}

impl InternalAlertingWebhookNotifierBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn method(mut self, value: AlertingWebhookMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn headers(mut self, value: Vec<AlertingWebhookHeader>) -> Self {
        self.headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InternalAlertingWebhookNotifier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](InternalAlertingWebhookNotifierBuilder::url)
    pub fn build(self) -> Result<InternalAlertingWebhookNotifier, BuildError> {
        Ok(InternalAlertingWebhookNotifier {
            r#type: self.r#type,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            method: self.method,
            headers: self.headers,
        })
    }
}
