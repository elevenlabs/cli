pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One segment of a source transcript.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingTranscriptSegment {
    /// Stable identifier of the segment, used to address it in edit requests.
    #[serde(default)]
    pub id: String,
    /// The transcribed text of the segment.
    #[serde(default)]
    pub text: String,
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
    /// The caller-supplied external ID for this segment, if one was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl DubbingTranscriptSegment {
    pub fn builder() -> DubbingTranscriptSegmentBuilder {
        <DubbingTranscriptSegmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTranscriptSegmentBuilder {
    id: Option<String>,
    text: Option<String>,
    speaker_id: Option<String>,
    start_s: Option<f64>,
    end_s: Option<f64>,
    external_id: Option<String>,
}

impl DubbingTranscriptSegmentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
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

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingTranscriptSegment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DubbingTranscriptSegmentBuilder::id)
    /// - [`text`](DubbingTranscriptSegmentBuilder::text)
    /// - [`speaker_id`](DubbingTranscriptSegmentBuilder::speaker_id)
    /// - [`start_s`](DubbingTranscriptSegmentBuilder::start_s)
    /// - [`end_s`](DubbingTranscriptSegmentBuilder::end_s)
    pub fn build(self) -> Result<DubbingTranscriptSegment, BuildError> {
        Ok(DubbingTranscriptSegment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            speaker_id: self.speaker_id.ok_or_else(|| BuildError::missing_field("speaker_id"))?,
            start_s: self.start_s.ok_or_else(|| BuildError::missing_field("start_s"))?,
            end_s: self.end_s.ok_or_else(|| BuildError::missing_field("end_s"))?,
            external_id: self.external_id,
        })
    }
}
