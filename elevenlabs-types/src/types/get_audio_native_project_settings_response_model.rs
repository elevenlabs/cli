pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAudioNativeProjectSettingsResponseModel {
    /// Whether the project is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// The ID of the latest snapshot of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// The settings of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AudioNativeProjectSettingsResponseModel>,
}

impl GetAudioNativeProjectSettingsResponseModel {
    pub fn builder() -> GetAudioNativeProjectSettingsResponseModelBuilder {
        <GetAudioNativeProjectSettingsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAudioNativeProjectSettingsResponseModelBuilder {
    enabled: Option<bool>,
    snapshot_id: Option<String>,
    settings: Option<AudioNativeProjectSettingsResponseModel>,
}

impl GetAudioNativeProjectSettingsResponseModelBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.snapshot_id = Some(value.into());
        self
    }

    pub fn settings(mut self, value: AudioNativeProjectSettingsResponseModel) -> Self {
        self.settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAudioNativeProjectSettingsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](GetAudioNativeProjectSettingsResponseModelBuilder::enabled)
    pub fn build(self) -> Result<GetAudioNativeProjectSettingsResponseModel, BuildError> {
        Ok(GetAudioNativeProjectSettingsResponseModel {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            snapshot_id: self.snapshot_id,
            settings: self.settings,
        })
    }
}
