pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TranscriptionOrderItemRequest {
    /// The IDs of the uploaded media files to transcribe.
    #[serde(default)]
    pub media_ids: Vec<MediaId>,
    /// The language code of the source media (e.g. 'en', 'es').
    #[serde(default)]
    pub source_language: String,
    /// Whether to transcribe every word exactly, including filler words.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbatim: Option<bool>,
    /// Optional free-text instructions for the transcription team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

impl TranscriptionOrderItemRequest {
    pub fn builder() -> TranscriptionOrderItemRequestBuilder {
        <TranscriptionOrderItemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptionOrderItemRequestBuilder {
    media_ids: Option<Vec<MediaId>>,
    source_language: Option<String>,
    verbatim: Option<bool>,
    instructions: Option<String>,
}

impl TranscriptionOrderItemRequestBuilder {
    pub fn media_ids(mut self, value: Vec<MediaId>) -> Self {
        self.media_ids = Some(value);
        self
    }

    pub fn source_language(mut self, value: impl Into<String>) -> Self {
        self.source_language = Some(value.into());
        self
    }

    pub fn verbatim(mut self, value: bool) -> Self {
        self.verbatim = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranscriptionOrderItemRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`media_ids`](TranscriptionOrderItemRequestBuilder::media_ids)
    /// - [`source_language`](TranscriptionOrderItemRequestBuilder::source_language)
    pub fn build(self) -> Result<TranscriptionOrderItemRequest, BuildError> {
        Ok(TranscriptionOrderItemRequest {
            media_ids: self.media_ids.ok_or_else(|| BuildError::missing_field("media_ids"))?,
            source_language: self.source_language.ok_or_else(|| BuildError::missing_field("source_language"))?,
            verbatim: self.verbatim,
            instructions: self.instructions,
        })
    }
}
