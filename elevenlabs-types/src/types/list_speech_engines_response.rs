pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSpeechEnginesResponse {
    /// The speech engines matching the query
    #[serde(default)]
    pub speech_engines: Vec<SpeechEngineSummaryResponse>,
    /// Cursor for fetching the next page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether there are more results
    #[serde(default)]
    pub has_more: bool,
}

impl ListSpeechEnginesResponse {
    pub fn builder() -> ListSpeechEnginesResponseBuilder {
        <ListSpeechEnginesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSpeechEnginesResponseBuilder {
    speech_engines: Option<Vec<SpeechEngineSummaryResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl ListSpeechEnginesResponseBuilder {
    pub fn speech_engines(mut self, value: Vec<SpeechEngineSummaryResponse>) -> Self {
        self.speech_engines = Some(value);
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

    /// Consumes the builder and constructs a [`ListSpeechEnginesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`speech_engines`](ListSpeechEnginesResponseBuilder::speech_engines)
    /// - [`has_more`](ListSpeechEnginesResponseBuilder::has_more)
    pub fn build(self) -> Result<ListSpeechEnginesResponse, BuildError> {
        Ok(ListSpeechEnginesResponse {
            speech_engines: self.speech_engines.ok_or_else(|| BuildError::missing_field("speech_engines"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
