pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubbingVoiceSettings {
    /// How strongly the dubbed speakers clone the source voices, 0 to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloning_strength: Option<i64>,
}

impl DubbingVoiceSettings {
    pub fn builder() -> DubbingVoiceSettingsBuilder {
        <DubbingVoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingVoiceSettingsBuilder {
    cloning_strength: Option<i64>,
}

impl DubbingVoiceSettingsBuilder {
    pub fn cloning_strength(mut self, value: i64) -> Self {
        self.cloning_strength = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingVoiceSettings`].
    pub fn build(self) -> Result<DubbingVoiceSettings, BuildError> {
        Ok(DubbingVoiceSettings {
            cloning_strength: self.cloning_strength,
        })
    }
}
