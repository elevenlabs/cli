pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetSpeechHistoryResponse {
    /// A list of speech history items.
    #[serde(default)]
    pub history: Vec<SpeechHistoryItemResponse>,
    /// The ID of the last history item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_history_item_id: Option<String>,
    /// Whether there are more history items to fetch.
    #[serde(default)]
    pub has_more: bool,
    /// The timestamp of the last history item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanned_until: Option<i64>,
}

impl GetSpeechHistoryResponse {
    pub fn builder() -> GetSpeechHistoryResponseBuilder {
        <GetSpeechHistoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSpeechHistoryResponseBuilder {
    history: Option<Vec<SpeechHistoryItemResponse>>,
    last_history_item_id: Option<String>,
    has_more: Option<bool>,
    scanned_until: Option<i64>,
}

impl GetSpeechHistoryResponseBuilder {
    pub fn history(mut self, value: Vec<SpeechHistoryItemResponse>) -> Self {
        self.history = Some(value);
        self
    }

    pub fn last_history_item_id(mut self, value: impl Into<String>) -> Self {
        self.last_history_item_id = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn scanned_until(mut self, value: i64) -> Self {
        self.scanned_until = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSpeechHistoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`history`](GetSpeechHistoryResponseBuilder::history)
    /// - [`has_more`](GetSpeechHistoryResponseBuilder::has_more)
    pub fn build(self) -> Result<GetSpeechHistoryResponse, BuildError> {
        Ok(GetSpeechHistoryResponse {
            history: self.history.ok_or_else(|| BuildError::missing_field("history"))?,
            last_history_item_id: self.last_history_item_id,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            scanned_until: self.scanned_until,
        })
    }
}
