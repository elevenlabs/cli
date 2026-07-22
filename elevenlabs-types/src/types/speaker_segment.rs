pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeakerSegment {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start_time: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end_time: f64,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub subtitles: Vec<SegmentSubtitleFrame>,
    #[serde(default)]
    pub dubs: HashMap<String, DubbedSegment>,
}

impl SpeakerSegment {
    pub fn builder() -> SpeakerSegmentBuilder {
        <SpeakerSegmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeakerSegmentBuilder {
    id: Option<String>,
    start_time: Option<f64>,
    end_time: Option<f64>,
    text: Option<String>,
    subtitles: Option<Vec<SegmentSubtitleFrame>>,
    dubs: Option<HashMap<String, DubbedSegment>>,
}

impl SpeakerSegmentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn dubs(mut self, value: HashMap<String, DubbedSegment>) -> Self {
        self.dubs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeakerSegment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SpeakerSegmentBuilder::id)
    /// - [`start_time`](SpeakerSegmentBuilder::start_time)
    /// - [`end_time`](SpeakerSegmentBuilder::end_time)
    /// - [`text`](SpeakerSegmentBuilder::text)
    /// - [`subtitles`](SpeakerSegmentBuilder::subtitles)
    /// - [`dubs`](SpeakerSegmentBuilder::dubs)
    pub fn build(self) -> Result<SpeakerSegment, BuildError> {
        Ok(SpeakerSegment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            start_time: self.start_time.ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self.end_time.ok_or_else(|| BuildError::missing_field("end_time"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            subtitles: self.subtitles.ok_or_else(|| BuildError::missing_field("subtitles"))?,
            dubs: self.dubs.ok_or_else(|| BuildError::missing_field("dubs"))?,
        })
    }
}
