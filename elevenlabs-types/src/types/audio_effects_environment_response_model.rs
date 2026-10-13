pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioEffectsEnvironmentResponseModel {
    /// Value for `audio_effects.environment_id`.
    #[serde(default)]
    pub environment_id: String,
    /// Display name.
    #[serde(default)]
    pub name: String,
    /// What the space sounds like.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AudioEffectsEnvironmentResponseModel {
    pub fn builder() -> AudioEffectsEnvironmentResponseModelBuilder {
        <AudioEffectsEnvironmentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioEffectsEnvironmentResponseModelBuilder {
    environment_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
}

impl AudioEffectsEnvironmentResponseModelBuilder {
    pub fn environment_id(mut self, value: impl Into<String>) -> Self {
        self.environment_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`AudioEffectsEnvironmentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment_id`](AudioEffectsEnvironmentResponseModelBuilder::environment_id)
    /// - [`name`](AudioEffectsEnvironmentResponseModelBuilder::name)
    pub fn build(self) -> Result<AudioEffectsEnvironmentResponseModel, BuildError> {
        Ok(AudioEffectsEnvironmentResponseModel {
            environment_id: self.environment_id.ok_or_else(|| BuildError::missing_field("environment_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
