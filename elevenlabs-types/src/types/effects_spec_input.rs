pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Filter preset, distance (proximity EQ), and environment (convolution reverb).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EffectsSpecInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_preset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_noise_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub send_level: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

impl EffectsSpecInput {
    pub fn builder() -> EffectsSpecInputBuilder {
        <EffectsSpecInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EffectsSpecInputBuilder {
    filter_preset_id: Option<String>,
    distance: Option<f64>,
    environment_id: Option<String>,
    background_noise_id: Option<String>,
    send_level: Option<f64>,
    seed: Option<i64>,
}

impl EffectsSpecInputBuilder {
    pub fn filter_preset_id(mut self, value: impl Into<String>) -> Self {
        self.filter_preset_id = Some(value.into());
        self
    }

    pub fn distance(mut self, value: f64) -> Self {
        self.distance = Some(value);
        self
    }

    pub fn environment_id(mut self, value: impl Into<String>) -> Self {
        self.environment_id = Some(value.into());
        self
    }

    pub fn background_noise_id(mut self, value: impl Into<String>) -> Self {
        self.background_noise_id = Some(value.into());
        self
    }

    pub fn send_level(mut self, value: f64) -> Self {
        self.send_level = Some(value);
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EffectsSpecInput`].
    pub fn build(self) -> Result<EffectsSpecInput, BuildError> {
        Ok(EffectsSpecInput {
            filter_preset_id: self.filter_preset_id,
            distance: self.distance,
            environment_id: self.environment_id,
            background_noise_id: self.background_noise_id,
            send_level: self.send_level,
            seed: self.seed,
        })
    }
}
