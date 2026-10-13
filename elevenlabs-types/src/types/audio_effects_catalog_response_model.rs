pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioEffectsCatalogResponseModel {
    #[serde(default)]
    pub filter_presets: Vec<AudioEffectsFilterPresetResponseModel>,
    #[serde(default)]
    pub environments: Vec<AudioEffectsEnvironmentResponseModel>,
    #[serde(default)]
    pub background_noises: Vec<AudioEffectsBackgroundNoiseResponseModel>,
}

impl AudioEffectsCatalogResponseModel {
    pub fn builder() -> AudioEffectsCatalogResponseModelBuilder {
        <AudioEffectsCatalogResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioEffectsCatalogResponseModelBuilder {
    filter_presets: Option<Vec<AudioEffectsFilterPresetResponseModel>>,
    environments: Option<Vec<AudioEffectsEnvironmentResponseModel>>,
    background_noises: Option<Vec<AudioEffectsBackgroundNoiseResponseModel>>,
}

impl AudioEffectsCatalogResponseModelBuilder {
    pub fn filter_presets(mut self, value: Vec<AudioEffectsFilterPresetResponseModel>) -> Self {
        self.filter_presets = Some(value);
        self
    }

    pub fn environments(mut self, value: Vec<AudioEffectsEnvironmentResponseModel>) -> Self {
        self.environments = Some(value);
        self
    }

    pub fn background_noises(mut self, value: Vec<AudioEffectsBackgroundNoiseResponseModel>) -> Self {
        self.background_noises = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioEffectsCatalogResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`filter_presets`](AudioEffectsCatalogResponseModelBuilder::filter_presets)
    /// - [`environments`](AudioEffectsCatalogResponseModelBuilder::environments)
    /// - [`background_noises`](AudioEffectsCatalogResponseModelBuilder::background_noises)
    pub fn build(self) -> Result<AudioEffectsCatalogResponseModel, BuildError> {
        Ok(AudioEffectsCatalogResponseModel {
            filter_presets: self.filter_presets.ok_or_else(|| BuildError::missing_field("filter_presets"))?,
            environments: self.environments.ok_or_else(|| BuildError::missing_field("environments"))?,
            background_noises: self.background_noises.ok_or_else(|| BuildError::missing_field("background_noises"))?,
        })
    }
}
