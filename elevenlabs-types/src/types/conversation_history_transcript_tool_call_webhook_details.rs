pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationHistoryTranscriptToolCallWebhookDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_params: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl ConversationHistoryTranscriptToolCallWebhookDetails {
    pub fn builder() -> ConversationHistoryTranscriptToolCallWebhookDetailsBuilder {
        <ConversationHistoryTranscriptToolCallWebhookDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptToolCallWebhookDetailsBuilder {
    r#type: Option<String>,
    method: Option<String>,
    url: Option<String>,
    headers: Option<HashMap<String, String>>,
    path_params: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
    body: Option<String>,
}

impl ConversationHistoryTranscriptToolCallWebhookDetailsBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn path_params(mut self, value: HashMap<String, String>) -> Self {
        self.path_params = Some(value);
        self
    }

    pub fn query_params(mut self, value: HashMap<String, String>) -> Self {
        self.query_params = Some(value);
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptToolCallWebhookDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](ConversationHistoryTranscriptToolCallWebhookDetailsBuilder::method)
    /// - [`url`](ConversationHistoryTranscriptToolCallWebhookDetailsBuilder::url)
    pub fn build(self) -> Result<ConversationHistoryTranscriptToolCallWebhookDetails, BuildError> {
        Ok(ConversationHistoryTranscriptToolCallWebhookDetails {
            r#type: self.r#type,
            method: self.method.ok_or_else(|| BuildError::missing_field("method"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            headers: self.headers,
            path_params: self.path_params,
            query_params: self.query_params,
            body: self.body,
        })
    }
}
