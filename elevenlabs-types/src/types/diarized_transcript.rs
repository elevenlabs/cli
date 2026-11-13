pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// With diarize, a committed transcript's words with their final speakers, sent after
/// its committed_transcript (which never waits for them), in commit order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DiarizedTranscript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// The segment_index of the committed_transcript the words are of.
    #[serde(default)]
    pub segment_index: i64,
    /// The committed transcript text the words are of.
    #[serde(default)]
    pub text: String,
    /// The words, as in batch diarization, each with its speaker_id, null where the speaker could not be identified.
    #[serde(default)]
    pub words: Vec<DiarizedTranscriptWordsItem>,
}

impl DiarizedTranscript {
    pub fn builder() -> DiarizedTranscriptBuilder {
        <DiarizedTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DiarizedTranscriptBuilder {
    message_type: Option<String>,
    segment_index: Option<i64>,
    text: Option<String>,
    words: Option<Vec<DiarizedTranscriptWordsItem>>,
}

impl DiarizedTranscriptBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn segment_index(mut self, value: i64) -> Self {
        self.segment_index = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn words(mut self, value: Vec<DiarizedTranscriptWordsItem>) -> Self {
        self.words = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DiarizedTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segment_index`](DiarizedTranscriptBuilder::segment_index)
    /// - [`text`](DiarizedTranscriptBuilder::text)
    /// - [`words`](DiarizedTranscriptBuilder::words)
    pub fn build(self) -> Result<DiarizedTranscript, BuildError> {
        Ok(DiarizedTranscript {
            message_type: self.message_type,
            segment_index: self.segment_index.ok_or_else(|| BuildError::missing_field("segment_index"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            words: self.words.ok_or_else(|| BuildError::missing_field("words"))?,
        })
    }
}
