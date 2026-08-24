pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplicateVoiceToIsolatedEnvironmentRequestModel {
    /// ID of the workspace to replicate the voice into. It must belong to the same consolidated billing group as the calling workspace; the target's data residency is derived from that link.
    #[serde(default)]
    pub target_workspace_id: String,
    /// When true (default) the replicated voice keeps the same voice ID in the target residency; set to false to assign a new voice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_voice_id: Option<bool>,
}

impl ReplicateVoiceToIsolatedEnvironmentRequestModel {
    pub fn builder() -> ReplicateVoiceToIsolatedEnvironmentRequestModelBuilder {
        <ReplicateVoiceToIsolatedEnvironmentRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplicateVoiceToIsolatedEnvironmentRequestModelBuilder {
    target_workspace_id: Option<String>,
    preserve_voice_id: Option<bool>,
}

impl ReplicateVoiceToIsolatedEnvironmentRequestModelBuilder {
    pub fn target_workspace_id(mut self, value: impl Into<String>) -> Self {
        self.target_workspace_id = Some(value.into());
        self
    }

    pub fn preserve_voice_id(mut self, value: bool) -> Self {
        self.preserve_voice_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplicateVoiceToIsolatedEnvironmentRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_workspace_id`](ReplicateVoiceToIsolatedEnvironmentRequestModelBuilder::target_workspace_id)
    pub fn build(self) -> Result<ReplicateVoiceToIsolatedEnvironmentRequestModel, BuildError> {
        Ok(ReplicateVoiceToIsolatedEnvironmentRequestModel {
            target_workspace_id: self.target_workspace_id.ok_or_else(|| BuildError::missing_field("target_workspace_id"))?,
            preserve_voice_id: self.preserve_voice_id,
        })
    }
}

