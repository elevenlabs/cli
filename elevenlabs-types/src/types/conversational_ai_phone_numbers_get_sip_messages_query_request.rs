pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_sip_messages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiPhoneNumbersGetSipMessagesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ConversationalAiPhoneNumbersGetSipMessagesQueryRequest {
    pub fn builder() -> ConversationalAiPhoneNumbersGetSipMessagesQueryRequestBuilder {
        <ConversationalAiPhoneNumbersGetSipMessagesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiPhoneNumbersGetSipMessagesQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl ConversationalAiPhoneNumbersGetSipMessagesQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiPhoneNumbersGetSipMessagesQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiPhoneNumbersGetSipMessagesQueryRequest, BuildError> {
        Ok(ConversationalAiPhoneNumbersGetSipMessagesQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

