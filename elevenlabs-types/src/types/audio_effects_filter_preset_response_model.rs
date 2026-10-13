pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioEffectsFilterPresetResponseModel {
    /// Value for `audio_effects.filter_preset_id`.
    #[serde(default)]
    pub filter_preset_id: String,
    /// Display name.
    #[serde(default)]
    pub name: String,
    /// What the preset sounds like.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AudioEffectsFilterPresetResponseModel {
    pub fn builder() -> AudioEffectsFilterPresetResponseModelBuilder {
        <AudioEffectsFilterPresetResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioEffectsFilterPresetResponseModelBuilder {
    filter_preset_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
}

impl AudioEffectsFilterPresetResponseModelBuilder {
    pub fn filter_preset_id(mut self, value: impl Into<String>) -> Self {
        self.filter_preset_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioEffectsFilterPresetResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`filter_preset_id`](AudioEffectsFilterPresetResponseModelBuilder::filter_preset_id)
    /// - [`name`](AudioEffectsFilterPresetResponseModelBuilder::name)
    pub fn build(self) -> Result<AudioEffectsFilterPresetResponseModel, BuildError> {
        Ok(AudioEffectsFilterPresetResponseModel {
            filter_preset_id: self.filter_preset_id.ok_or_else(|| BuildError::missing_field("filter_preset_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
