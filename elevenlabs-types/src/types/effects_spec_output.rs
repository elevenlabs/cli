pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Audio effects applied to generated speech: a filter preset, perceived distance, a reverb
/// environment, a background noise bed, and stereo panning. Leave every field at its default
/// to get unprocessed audio.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EffectsSpecOutput {
    /// ID of a filter preset that colors the voice, such as `phone` or `old_radio`. List valid IDs with `GET /v1/audio-effects/catalog`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_preset_id: Option<String>,
    /// How far the speaker sounds from the microphone, from 0 (close, unchanged) to 1 (far). Higher values roll off low and high frequencies and, when an environment is set, shift the balance from the dry voice toward the reverb.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub distance: f64,
    /// ID of a reverb environment that places the voice in a space, such as `small_room` or `hall`. List valid IDs with `GET /v1/audio-effects/catalog`. Reverb is rendered in stereo and adds a tail of up to several seconds after the speech ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// ID of a background noise bed looped under the voice, such as `cafe` or `keyboard`. List valid IDs with `GET /v1/audio-effects/catalog`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_noise_id: Option<String>,
    /// How much of the voice feeds the reverb, from 0 (none) to 1 (full). Only applies when `environment_id` is set; 0 turns the reverb off.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub send_level: f64,
    /// Stereo position of the voice, from -1 (hard left) through 0 (center) to 1 (hard right). Only the voice is panned; reverb and background noise stay centered.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub pan: f64,
    /// Has no effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
}

impl EffectsSpecOutput {
    pub fn builder() -> EffectsSpecOutputBuilder {
        <EffectsSpecOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EffectsSpecOutputBuilder {
    filter_preset_id: Option<String>,
    distance: Option<f64>,
    environment_id: Option<String>,
    background_noise_id: Option<String>,
    send_level: Option<f64>,
    pan: Option<f64>,
    seed: Option<i64>,
}

impl EffectsSpecOutputBuilder {
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

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EffectsSpecOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`distance`](EffectsSpecOutputBuilder::distance)
    /// - [`send_level`](EffectsSpecOutputBuilder::send_level)
    /// - [`pan`](EffectsSpecOutputBuilder::pan)
    pub fn build(self) -> Result<EffectsSpecOutput, BuildError> {
        Ok(EffectsSpecOutput {
            filter_preset_id: self.filter_preset_id,
            distance: self.distance.ok_or_else(|| BuildError::missing_field("distance"))?,
            environment_id: self.environment_id,
            background_noise_id: self.background_noise_id,
            send_level: self.send_level.ok_or_else(|| BuildError::missing_field("send_level"))?,
            pan: self.pan.ok_or_else(|| BuildError::missing_field("pan"))?,
            seed: self.seed,
        })
    }
}
