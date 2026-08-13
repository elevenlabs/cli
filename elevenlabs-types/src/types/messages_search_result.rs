pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// transcript_index: index of the message in the conversation transcript
/// chunk_text: text of the transcript; transcript messages if very long could have several chunks.
/// chunk_highlights: chunk_text split into matched/unmatched segments for highlighting.
/// Only populated for keyword/text search, not semantic search.
/// score: similarity score of the message to the search query
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MessagesSearchResult {
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(default)]
    pub transcript_index: i64,
    #[serde(default)]
    pub chunk_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_highlights: Option<Vec<SearchHighlightSegment>>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score: f64,
    #[serde(default)]
    pub conversation_start_time_unix_secs: i64,
}

impl MessagesSearchResult {
    pub fn builder() -> MessagesSearchResultBuilder {
        <MessagesSearchResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessagesSearchResultBuilder {
    conversation_id: Option<String>,
    agent_id: Option<String>,
    agent_name: Option<String>,
    transcript_index: Option<i64>,
    chunk_text: Option<String>,
    chunk_highlights: Option<Vec<SearchHighlightSegment>>,
    score: Option<f64>,
    conversation_start_time_unix_secs: Option<i64>,
}

impl MessagesSearchResultBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn transcript_index(mut self, value: i64) -> Self {
        self.transcript_index = Some(value);
        self
    }

    pub fn chunk_text(mut self, value: impl Into<String>) -> Self {
        self.chunk_text = Some(value.into());
        self
    }

    pub fn chunk_highlights(mut self, value: Vec<SearchHighlightSegment>) -> Self {
        self.chunk_highlights = Some(value);
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn conversation_start_time_unix_secs(mut self, value: i64) -> Self {
        self.conversation_start_time_unix_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessagesSearchResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](MessagesSearchResultBuilder::conversation_id)
    /// - [`agent_id`](MessagesSearchResultBuilder::agent_id)
    /// - [`transcript_index`](MessagesSearchResultBuilder::transcript_index)
    /// - [`chunk_text`](MessagesSearchResultBuilder::chunk_text)
    /// - [`score`](MessagesSearchResultBuilder::score)
    /// - [`conversation_start_time_unix_secs`](MessagesSearchResultBuilder::conversation_start_time_unix_secs)
    pub fn build(self) -> Result<MessagesSearchResult, BuildError> {
        Ok(MessagesSearchResult {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_name: self.agent_name,
            transcript_index: self.transcript_index.ok_or_else(|| BuildError::missing_field("transcript_index"))?,
            chunk_text: self.chunk_text.ok_or_else(|| BuildError::missing_field("chunk_text"))?,
            chunk_highlights: self.chunk_highlights,
            score: self.score.ok_or_else(|| BuildError::missing_field("score"))?,
            conversation_start_time_unix_secs: self.conversation_start_time_unix_secs.ok_or_else(|| BuildError::missing_field("conversation_start_time_unix_secs"))?,
        })
    }
}
