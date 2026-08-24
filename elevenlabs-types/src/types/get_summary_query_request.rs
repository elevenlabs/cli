pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_summary
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSummaryQueryRequest {
    /// Maximum number of chat message turns to include inline. When the conversation has more than this, the messages are omitted and messages_omitted is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_messages: Option<i64>,
}

impl GetSummaryQueryRequest {
    pub fn builder() -> GetSummaryQueryRequestBuilder {
        <GetSummaryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSummaryQueryRequestBuilder {
    max_messages: Option<i64>,
}

impl GetSummaryQueryRequestBuilder {
    pub fn max_messages(mut self, value: i64) -> Self {
        self.max_messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSummaryQueryRequest`].
    pub fn build(self) -> Result<GetSummaryQueryRequest, BuildError> {
        Ok(GetSummaryQueryRequest {
            max_messages: self.max_messages,
        })
    }
}

