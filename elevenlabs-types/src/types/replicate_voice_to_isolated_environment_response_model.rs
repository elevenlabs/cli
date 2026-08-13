pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplicateVoiceToIsolatedEnvironmentResponseModel {
    /// Voice ID of the replicated voice in the target residency.
    #[serde(default)]
    pub voice_id: String,
}

impl ReplicateVoiceToIsolatedEnvironmentResponseModel {
    pub fn builder() -> ReplicateVoiceToIsolatedEnvironmentResponseModelBuilder {
        <ReplicateVoiceToIsolatedEnvironmentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplicateVoiceToIsolatedEnvironmentResponseModelBuilder {
    voice_id: Option<String>,
}

impl ReplicateVoiceToIsolatedEnvironmentResponseModelBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReplicateVoiceToIsolatedEnvironmentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](ReplicateVoiceToIsolatedEnvironmentResponseModelBuilder::voice_id)
    pub fn build(self) -> Result<ReplicateVoiceToIsolatedEnvironmentResponseModel, BuildError> {
        Ok(ReplicateVoiceToIsolatedEnvironmentResponseModel {
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
        })
    }
}
