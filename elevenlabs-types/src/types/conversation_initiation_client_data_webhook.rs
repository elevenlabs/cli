pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationInitiationClientDataWebhook {
    /// The URL to send the webhook to
    #[serde(default)]
    pub url: String,
    /// The headers to send with the webhook request
    #[serde(default)]
    pub request_headers: HashMap<String, ConversationInitiationClientDataWebhookRequestHeadersValue>,
}

impl ConversationInitiationClientDataWebhook {
    pub fn builder() -> ConversationInitiationClientDataWebhookBuilder {
        <ConversationInitiationClientDataWebhookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationInitiationClientDataWebhookBuilder {
    url: Option<String>,
    request_headers: Option<HashMap<String, ConversationInitiationClientDataWebhookRequestHeadersValue>>,
}

impl ConversationInitiationClientDataWebhookBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn request_headers(mut self, value: HashMap<String, ConversationInitiationClientDataWebhookRequestHeadersValue>) -> Self {
        self.request_headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationInitiationClientDataWebhook`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](ConversationInitiationClientDataWebhookBuilder::url)
    /// - [`request_headers`](ConversationInitiationClientDataWebhookBuilder::request_headers)
    pub fn build(self) -> Result<ConversationInitiationClientDataWebhook, BuildError> {
        Ok(ConversationInitiationClientDataWebhook {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            request_headers: self.request_headers.ok_or_else(|| BuildError::missing_field("request_headers"))?,
        })
    }
}
