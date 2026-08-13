pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BackgroundSoundConfig {
    /// The type of background sound source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<BackgroundSoundSourceType>,
    /// Identifier for the sound source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<BackgroundSoundPresetId>,
    /// Volume level for background sound (0.01 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume: Option<f64>,
    /// Apply a crossfade at the loop boundary to avoid audible pops when the sound loops.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossfade_loop: Option<bool>,
}

impl BackgroundSoundConfig {
    pub fn builder() -> BackgroundSoundConfigBuilder {
        <BackgroundSoundConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BackgroundSoundConfigBuilder {
    source_type: Option<BackgroundSoundSourceType>,
    source_id: Option<BackgroundSoundPresetId>,
    volume: Option<f64>,
    crossfade_loop: Option<bool>,
}

impl BackgroundSoundConfigBuilder {
    pub fn source_type(mut self, value: BackgroundSoundSourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    pub fn source_id(mut self, value: BackgroundSoundPresetId) -> Self {
        self.source_id = Some(value);
        self
    }

    pub fn volume(mut self, value: f64) -> Self {
        self.volume = Some(value);
        self
    }

    pub fn crossfade_loop(mut self, value: bool) -> Self {
        self.crossfade_loop = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BackgroundSoundConfig`].
    pub fn build(self) -> Result<BackgroundSoundConfig, BuildError> {
        Ok(BackgroundSoundConfig {
            source_type: self.source_type,
            source_id: self.source_id,
            volume: self.volume,
            crossfade_loop: self.crossfade_loop,
        })
    }
}
