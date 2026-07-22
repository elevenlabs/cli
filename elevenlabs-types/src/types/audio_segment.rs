pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioSegment {
    #[serde(default)]
    pub start_ms: i64,
    #[serde(default)]
    pub end_ms: i64,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_speech: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_music: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacing: Option<String>,
}

impl AudioSegment {
    pub fn builder() -> AudioSegmentBuilder {
        <AudioSegmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSegmentBuilder {
    start_ms: Option<i64>,
    end_ms: Option<i64>,
    description: Option<String>,
    segment_type: Option<String>,
    has_speech: Option<bool>,
    has_music: Option<bool>,
    pacing: Option<String>,
}

impl AudioSegmentBuilder {
    pub fn start_ms(mut self, value: i64) -> Self {
        self.start_ms = Some(value);
        self
    }

    pub fn end_ms(mut self, value: i64) -> Self {
        self.end_ms = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn segment_type(mut self, value: impl Into<String>) -> Self {
        self.segment_type = Some(value.into());
        self
    }

    pub fn has_speech(mut self, value: bool) -> Self {
        self.has_speech = Some(value);
        self
    }

    pub fn has_music(mut self, value: bool) -> Self {
        self.has_music = Some(value);
        self
    }

    pub fn pacing(mut self, value: impl Into<String>) -> Self {
        self.pacing = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioSegment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_ms`](AudioSegmentBuilder::start_ms)
    /// - [`end_ms`](AudioSegmentBuilder::end_ms)
    /// - [`description`](AudioSegmentBuilder::description)
    pub fn build(self) -> Result<AudioSegment, BuildError> {
        Ok(AudioSegment {
            start_ms: self.start_ms.ok_or_else(|| BuildError::missing_field("start_ms"))?,
            end_ms: self.end_ms.ok_or_else(|| BuildError::missing_field("end_ms"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            segment_type: self.segment_type,
            has_speech: self.has_speech,
            has_music: self.has_music,
            pacing: self.pacing,
        })
    }
}
