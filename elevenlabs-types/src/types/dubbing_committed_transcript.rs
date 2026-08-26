pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DubbingCommittedTranscript {
    pub message_type: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub segment_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start_s: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end_s: f64,
    #[serde(default)]
    pub speaker_id: String,
}

impl DubbingCommittedTranscript {
    pub fn builder() -> DubbingCommittedTranscriptBuilder {
        <DubbingCommittedTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingCommittedTranscriptBuilder {
    message_type: Option<String>,
    text: Option<String>,
    segment_id: Option<String>,
    start_s: Option<f64>,
    end_s: Option<f64>,
    speaker_id: Option<String>,
}

impl DubbingCommittedTranscriptBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn segment_id(mut self, value: impl Into<String>) -> Self {
        self.segment_id = Some(value.into());
        self
    }

    pub fn start_s(mut self, value: f64) -> Self {
        self.start_s = Some(value);
        self
    }

    pub fn end_s(mut self, value: f64) -> Self {
        self.end_s = Some(value);
        self
    }

    pub fn speaker_id(mut self, value: impl Into<String>) -> Self {
        self.speaker_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingCommittedTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](DubbingCommittedTranscriptBuilder::message_type)
    /// - [`text`](DubbingCommittedTranscriptBuilder::text)
    /// - [`segment_id`](DubbingCommittedTranscriptBuilder::segment_id)
    /// - [`start_s`](DubbingCommittedTranscriptBuilder::start_s)
    /// - [`end_s`](DubbingCommittedTranscriptBuilder::end_s)
    /// - [`speaker_id`](DubbingCommittedTranscriptBuilder::speaker_id)
    pub fn build(self) -> Result<DubbingCommittedTranscript, BuildError> {
        Ok(DubbingCommittedTranscript {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            segment_id: self.segment_id.ok_or_else(|| BuildError::missing_field("segment_id"))?,
            start_s: self.start_s.ok_or_else(|| BuildError::missing_field("start_s"))?,
            end_s: self.end_s.ok_or_else(|| BuildError::missing_field("end_s"))?,
            speaker_id: self.speaker_id.ok_or_else(|| BuildError::missing_field("speaker_id"))?,
        })
    }
}
