pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbedSegment {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start_time: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end_time: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default)]
    pub subtitles: Vec<SegmentSubtitleFrame>,
    #[serde(default)]
    pub audio_stale: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ref: Option<DubbingMediaReference>,
}

impl DubbedSegment {
    pub fn builder() -> DubbedSegmentBuilder {
        <DubbedSegmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbedSegmentBuilder {
    start_time: Option<f64>,
    end_time: Option<f64>,
    text: Option<String>,
    subtitles: Option<Vec<SegmentSubtitleFrame>>,
    audio_stale: Option<bool>,
    media_ref: Option<DubbingMediaReference>,
}

impl DubbedSegmentBuilder {
    pub fn start_time(mut self, value: f64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: f64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn subtitles(mut self, value: Vec<SegmentSubtitleFrame>) -> Self {
        self.subtitles = Some(value);
        self
    }

    pub fn audio_stale(mut self, value: bool) -> Self {
        self.audio_stale = Some(value);
        self
    }

    pub fn media_ref(mut self, value: DubbingMediaReference) -> Self {
        self.media_ref = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbedSegment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](DubbedSegmentBuilder::start_time)
    /// - [`end_time`](DubbedSegmentBuilder::end_time)
    /// - [`subtitles`](DubbedSegmentBuilder::subtitles)
    /// - [`audio_stale`](DubbedSegmentBuilder::audio_stale)
    pub fn build(self) -> Result<DubbedSegment, BuildError> {
        Ok(DubbedSegment {
            start_time: self.start_time.ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self.end_time.ok_or_else(|| BuildError::missing_field("end_time"))?,
            text: self.text,
            subtitles: self.subtitles.ok_or_else(|| BuildError::missing_field("subtitles"))?,
            audio_stale: self.audio_stale.ok_or_else(|| BuildError::missing_field("audio_stale"))?,
            media_ref: self.media_ref,
        })
    }
}
