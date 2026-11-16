pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Settings for creating an HMAC-authenticated webhook
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebhookHmacSettings {
    /// The authentication type for this webhook
    pub auth_type: String,
    /// The display name for this webhook
    #[serde(default)]
    pub name: String,
    /// The HTTPS callback URL that will be called when this webhook is triggered
    #[serde(default)]
    pub webhook_url: String,
    /// Optional custom request headers to include with each webhook delivery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<HashMap<String, Option<String>>>,
    /// Field names whose values the delivery log hides when it shows the request payload, matched at any depth, ignoring case and punctuation such as _ or -. Fields with secret-looking names, such as tokens and passwords, are always hidden. The stored log and redeliveries keep the original values. At most 50 names, each 1 to 128 characters, with no whitespace, dots or wildcards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_redacted_fields: Option<Vec<String>>,
}

impl WebhookHmacSettings {
    pub fn builder() -> WebhookHmacSettingsBuilder {
        <WebhookHmacSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookHmacSettingsBuilder {
    auth_type: Option<String>,
    name: Option<String>,
    webhook_url: Option<String>,
    request_headers: Option<HashMap<String, Option<String>>>,
    log_redacted_fields: Option<Vec<String>>,
}

impl WebhookHmacSettingsBuilder {
    pub fn auth_type(mut self, value: impl Into<String>) -> Self {
        self.auth_type = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn webhook_url(mut self, value: impl Into<String>) -> Self {
        self.webhook_url = Some(value.into());
        self
    }

    pub fn request_headers(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.request_headers = Some(value);
        self
    }

    pub fn log_redacted_fields(mut self, value: Vec<String>) -> Self {
        self.log_redacted_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookHmacSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_type`](WebhookHmacSettingsBuilder::auth_type)
    /// - [`name`](WebhookHmacSettingsBuilder::name)
    /// - [`webhook_url`](WebhookHmacSettingsBuilder::webhook_url)
    pub fn build(self) -> Result<WebhookHmacSettings, BuildError> {
        Ok(WebhookHmacSettings {
            auth_type: self.auth_type.ok_or_else(|| BuildError::missing_field("auth_type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            webhook_url: self.webhook_url.ok_or_else(|| BuildError::missing_field("webhook_url"))?,
            request_headers: self.request_headers,
            log_redacted_fields: self.log_redacted_fields,
        })
    }
}
