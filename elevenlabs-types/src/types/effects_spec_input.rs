pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Audio effects applied to generated speech: a filter preset, perceived distance, a reverb
/// environment, a background noise bed, and stereo panning. Leave every field at its default
/// to get unprocessed audio.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EffectsSpecInput {
    /// ID of a filter preset that colors the voice, such as `phone` or `old_radio`. List valid IDs with `GET /v1/audio-effects/catalog`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_preset_id: Option<String>,
    /// How far the speaker sounds from the microphone, from 0 (close, unchanged) to 1 (far). Higher values roll off low and high frequencies and, when an environment is set, shift the balance from the dry voice toward the reverb.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub distance: Option<f64>,
    /// ID of a reverb environment that places the voice in a space, such as `small_room` or `hall`. List valid IDs with `GET /v1/audio-effects/catalog`. Reverb is rendered in stereo and adds a tail of up to several seconds after the speech ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// ID of a background noise bed looped under the voice, such as `cafe` or `keyboard`. List valid IDs with `GET /v1/audio-effects/catalog`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_noise_id: Option<String>,
    /// How much of the voice feeds the reverb, from 0 (none) to 1 (full). Only applies when `environment_id` is set; 0 turns the reverb off.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub send_level: Option<f64>,
    /// Stereo position of the voice, from -1 (hard left) through 0 (center) to 1 (hard right). Only the voice is panned; reverb and background noise stay centered.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pan: Option<f64>,
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
    pan: Option<f64>,
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

    pub fn pan(mut self, value: f64) -> Self {
        self.pan = Some(value);
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
            pan: self.pan,
        })
    }
}
