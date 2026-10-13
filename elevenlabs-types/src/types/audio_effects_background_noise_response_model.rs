pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioEffectsBackgroundNoiseResponseModel {
    /// Value for `audio_effects.background_noise_id`.
    #[serde(default)]
    pub background_noise_id: String,
    /// Display name.
    #[serde(default)]
    pub name: String,
    /// What the noise bed sounds like.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AudioEffectsBackgroundNoiseResponseModel {
    pub fn builder() -> AudioEffectsBackgroundNoiseResponseModelBuilder {
        <AudioEffectsBackgroundNoiseResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioEffectsBackgroundNoiseResponseModelBuilder {
    background_noise_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
}

impl AudioEffectsBackgroundNoiseResponseModelBuilder {
    pub fn background_noise_id(mut self, value: impl Into<String>) -> Self {
        self.background_noise_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`AudioEffectsBackgroundNoiseResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`background_noise_id`](AudioEffectsBackgroundNoiseResponseModelBuilder::background_noise_id)
    /// - [`name`](AudioEffectsBackgroundNoiseResponseModelBuilder::name)
    pub fn build(self) -> Result<AudioEffectsBackgroundNoiseResponseModel, BuildError> {
        Ok(AudioEffectsBackgroundNoiseResponseModel {
            background_noise_id: self.background_noise_id.ok_or_else(|| BuildError::missing_field("background_noise_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
