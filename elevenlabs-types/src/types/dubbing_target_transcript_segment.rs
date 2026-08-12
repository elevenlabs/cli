pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One segment of a target transcript: a source segment plus its translation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingTargetTranscriptSegment {
    /// Stable identifier of the segment (from the source).
    #[serde(default)]
    pub id: String,
    /// Identifier of the segment's speaker.
    #[serde(default)]
    pub speaker_id: String,
    /// Start time of the segment, in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start_s: f64,
    /// End time of the segment, in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end_s: f64,
    /// The source-language text of the segment.
    #[serde(default)]
    pub source_text: String,
    /// The translated text, or null if not translated yet (needs translation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
}

impl DubbingTargetTranscriptSegment {
    pub fn builder() -> DubbingTargetTranscriptSegmentBuilder {
        <DubbingTargetTranscriptSegmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTargetTranscriptSegmentBuilder {
    id: Option<String>,
    speaker_id: Option<String>,
    start_s: Option<f64>,
    end_s: Option<f64>,
    source_text: Option<String>,
    translation: Option<String>,
}

impl DubbingTargetTranscriptSegmentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn speaker_id(mut self, value: impl Into<String>) -> Self {
        self.speaker_id = Some(value.into());
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

    pub fn source_text(mut self, value: impl Into<String>) -> Self {
        self.source_text = Some(value.into());
        self
    }

    pub fn translation(mut self, value: impl Into<String>) -> Self {
        self.translation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingTargetTranscriptSegment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DubbingTargetTranscriptSegmentBuilder::id)
    /// - [`speaker_id`](DubbingTargetTranscriptSegmentBuilder::speaker_id)
    /// - [`start_s`](DubbingTargetTranscriptSegmentBuilder::start_s)
    /// - [`end_s`](DubbingTargetTranscriptSegmentBuilder::end_s)
    /// - [`source_text`](DubbingTargetTranscriptSegmentBuilder::source_text)
    pub fn build(self) -> Result<DubbingTargetTranscriptSegment, BuildError> {
        Ok(DubbingTargetTranscriptSegment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            speaker_id: self.speaker_id.ok_or_else(|| BuildError::missing_field("speaker_id"))?,
            start_s: self.start_s.ok_or_else(|| BuildError::missing_field("start_s"))?,
            end_s: self.end_s.ok_or_else(|| BuildError::missing_field("end_s"))?,
            source_text: self.source_text.ok_or_else(|| BuildError::missing_field("source_text"))?,
            translation: self.translation,
        })
    }
}
