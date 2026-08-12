pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PendingExternalAudiosMetadataModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_global_offset_ms: Option<i64>,
    #[serde(default)]
    pub external_audio_ids: Vec<String>,
}

impl PendingExternalAudiosMetadataModel {
    pub fn builder() -> PendingExternalAudiosMetadataModelBuilder {
        <PendingExternalAudiosMetadataModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PendingExternalAudiosMetadataModelBuilder {
    target_global_offset_ms: Option<i64>,
    external_audio_ids: Option<Vec<String>>,
}

impl PendingExternalAudiosMetadataModelBuilder {
    pub fn target_global_offset_ms(mut self, value: i64) -> Self {
        self.target_global_offset_ms = Some(value);
        self
    }

    pub fn external_audio_ids(mut self, value: Vec<String>) -> Self {
        self.external_audio_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PendingExternalAudiosMetadataModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_audio_ids`](PendingExternalAudiosMetadataModelBuilder::external_audio_ids)
    pub fn build(self) -> Result<PendingExternalAudiosMetadataModel, BuildError> {
        Ok(PendingExternalAudiosMetadataModel {
            target_global_offset_ms: self.target_global_offset_ms,
            external_audio_ids: self.external_audio_ids.ok_or_else(|| BuildError::missing_field("external_audio_ids"))?,
        })
    }
}
