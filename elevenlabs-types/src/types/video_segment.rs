pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VideoSegment {
    #[serde(default)]
    pub start_ms: i64,
    #[serde(default)]
    pub end_ms: i64,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_movement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_speech: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_music: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacing: Option<String>,
}

impl VideoSegment {
    pub fn builder() -> VideoSegmentBuilder {
        <VideoSegmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VideoSegmentBuilder {
    start_ms: Option<i64>,
    end_ms: Option<i64>,
    description: Option<String>,
    subjects: Option<Vec<String>>,
    shot_type: Option<String>,
    camera_movement: Option<String>,
    transition_in: Option<String>,
    has_speech: Option<bool>,
    has_music: Option<bool>,
    pacing: Option<String>,
}

impl VideoSegmentBuilder {
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

    pub fn subjects(mut self, value: Vec<String>) -> Self {
        self.subjects = Some(value);
        self
    }

    pub fn shot_type(mut self, value: impl Into<String>) -> Self {
        self.shot_type = Some(value.into());
        self
    }

    pub fn camera_movement(mut self, value: impl Into<String>) -> Self {
        self.camera_movement = Some(value.into());
        self
    }

    pub fn transition_in(mut self, value: impl Into<String>) -> Self {
        self.transition_in = Some(value.into());
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

    /// Consumes the builder and constructs a [`VideoSegment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_ms`](VideoSegmentBuilder::start_ms)
    /// - [`end_ms`](VideoSegmentBuilder::end_ms)
    /// - [`description`](VideoSegmentBuilder::description)
    pub fn build(self) -> Result<VideoSegment, BuildError> {
        Ok(VideoSegment {
            start_ms: self.start_ms.ok_or_else(|| BuildError::missing_field("start_ms"))?,
            end_ms: self.end_ms.ok_or_else(|| BuildError::missing_field("end_ms"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            subjects: self.subjects,
            shot_type: self.shot_type,
            camera_movement: self.camera_movement,
            transition_in: self.transition_in,
            has_speech: self.has_speech,
            has_music: self.has_music,
            pacing: self.pacing,
        })
    }
}
