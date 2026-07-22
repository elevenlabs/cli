pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceSegment {
    /// The voice ID used for this segment
    #[serde(default)]
    pub voice_id: String,
    /// Start time of this voice segment
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub start_time_seconds: f64,
    /// End time of this voice segment
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub end_time_seconds: f64,
    /// Start index in the characters array
    #[serde(default)]
    pub character_start_index: i64,
    /// End index in the characters array (exclusive)
    #[serde(default)]
    pub character_end_index: i64,
    /// Line of the dialogue (script) that this segment is a part of.
    #[serde(default)]
    pub dialogue_input_index: i64,
}

impl VoiceSegment {
    pub fn builder() -> VoiceSegmentBuilder {
        <VoiceSegmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceSegmentBuilder {
    voice_id: Option<String>,
    start_time_seconds: Option<f64>,
    end_time_seconds: Option<f64>,
    character_start_index: Option<i64>,
    character_end_index: Option<i64>,
    dialogue_input_index: Option<i64>,
}

impl VoiceSegmentBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn start_time_seconds(mut self, value: f64) -> Self {
        self.start_time_seconds = Some(value);
        self
    }

    pub fn end_time_seconds(mut self, value: f64) -> Self {
        self.end_time_seconds = Some(value);
        self
    }

    pub fn character_start_index(mut self, value: i64) -> Self {
        self.character_start_index = Some(value);
        self
    }

    pub fn character_end_index(mut self, value: i64) -> Self {
        self.character_end_index = Some(value);
        self
    }

    pub fn dialogue_input_index(mut self, value: i64) -> Self {
        self.dialogue_input_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceSegment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](VoiceSegmentBuilder::voice_id)
    /// - [`start_time_seconds`](VoiceSegmentBuilder::start_time_seconds)
    /// - [`end_time_seconds`](VoiceSegmentBuilder::end_time_seconds)
    /// - [`character_start_index`](VoiceSegmentBuilder::character_start_index)
    /// - [`character_end_index`](VoiceSegmentBuilder::character_end_index)
    /// - [`dialogue_input_index`](VoiceSegmentBuilder::dialogue_input_index)
    pub fn build(self) -> Result<VoiceSegment, BuildError> {
        Ok(VoiceSegment {
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            start_time_seconds: self.start_time_seconds.ok_or_else(|| BuildError::missing_field("start_time_seconds"))?,
            end_time_seconds: self.end_time_seconds.ok_or_else(|| BuildError::missing_field("end_time_seconds"))?,
            character_start_index: self.character_start_index.ok_or_else(|| BuildError::missing_field("character_start_index"))?,
            character_end_index: self.character_end_index.ok_or_else(|| BuildError::missing_field("character_end_index"))?,
            dialogue_input_index: self.dialogue_input_index.ok_or_else(|| BuildError::missing_field("dialogue_input_index"))?,
        })
    }
}
