pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSipLogMessagesResponse {
    #[serde(default)]
    pub sip_messages: Vec<SipLogMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl GetSipLogMessagesResponse {
    pub fn builder() -> GetSipLogMessagesResponseBuilder {
        <GetSipLogMessagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSipLogMessagesResponseBuilder {
    sip_messages: Option<Vec<SipLogMessage>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetSipLogMessagesResponseBuilder {
    pub fn sip_messages(mut self, value: Vec<SipLogMessage>) -> Self {
        self.sip_messages = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSipLogMessagesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sip_messages`](GetSipLogMessagesResponseBuilder::sip_messages)
    pub fn build(self) -> Result<GetSipLogMessagesResponse, BuildError> {
        Ok(GetSipLogMessagesResponse {
            sip_messages: self.sip_messages.ok_or_else(|| BuildError::missing_field("sip_messages"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
