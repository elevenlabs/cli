pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoiceStatisticsResponseModel {
    /// The project voice reference ID.
    #[serde(default)]
    pub project_voice_ref_id: String,
    /// The number of unconverted characters for this voice.
    #[serde(default)]
    pub characters_unconverted: i64,
    /// The number of converted characters for this voice.
    #[serde(default)]
    pub characters_converted: i64,
    /// The number of credits needed to convert the remaining audio for this voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits_needed_to_convert: Option<i64>,
    /// The voice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

impl VoiceStatisticsResponseModel {
    pub fn builder() -> VoiceStatisticsResponseModelBuilder {
        <VoiceStatisticsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceStatisticsResponseModelBuilder {
    project_voice_ref_id: Option<String>,
    characters_unconverted: Option<i64>,
    characters_converted: Option<i64>,
    credits_needed_to_convert: Option<i64>,
    voice_id: Option<String>,
}

impl VoiceStatisticsResponseModelBuilder {
    pub fn project_voice_ref_id(mut self, value: impl Into<String>) -> Self {
        self.project_voice_ref_id = Some(value.into());
        self
    }

    pub fn characters_unconverted(mut self, value: i64) -> Self {
        self.characters_unconverted = Some(value);
        self
    }

    pub fn characters_converted(mut self, value: i64) -> Self {
        self.characters_converted = Some(value);
        self
    }

    pub fn credits_needed_to_convert(mut self, value: i64) -> Self {
        self.credits_needed_to_convert = Some(value);
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoiceStatisticsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_voice_ref_id`](VoiceStatisticsResponseModelBuilder::project_voice_ref_id)
    /// - [`characters_unconverted`](VoiceStatisticsResponseModelBuilder::characters_unconverted)
    /// - [`characters_converted`](VoiceStatisticsResponseModelBuilder::characters_converted)
    pub fn build(self) -> Result<VoiceStatisticsResponseModel, BuildError> {
        Ok(VoiceStatisticsResponseModel {
            project_voice_ref_id: self.project_voice_ref_id.ok_or_else(|| BuildError::missing_field("project_voice_ref_id"))?,
            characters_unconverted: self.characters_unconverted.ok_or_else(|| BuildError::missing_field("characters_unconverted"))?,
            characters_converted: self.characters_converted.ok_or_else(|| BuildError::missing_field("characters_converted"))?,
            credits_needed_to_convert: self.credits_needed_to_convert,
            voice_id: self.voice_id,
        })
    }
}
