pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAgentHoldAudioResponseModel {
    #[serde(default)]
    pub agent_id: String,
}

impl DeleteAgentHoldAudioResponseModel {
    pub fn builder() -> DeleteAgentHoldAudioResponseModelBuilder {
        <DeleteAgentHoldAudioResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAgentHoldAudioResponseModelBuilder {
    agent_id: Option<String>,
}

impl DeleteAgentHoldAudioResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAgentHoldAudioResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](DeleteAgentHoldAudioResponseModelBuilder::agent_id)
    pub fn build(self) -> Result<DeleteAgentHoldAudioResponseModel, BuildError> {
        Ok(DeleteAgentHoldAudioResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
        })
    }
}
