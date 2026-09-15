pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostAgentHoldAudioResponseModel {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub hold_audio: AgentHoldAudioConfig,
}

impl PostAgentHoldAudioResponseModel {
    pub fn builder() -> PostAgentHoldAudioResponseModelBuilder {
        <PostAgentHoldAudioResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostAgentHoldAudioResponseModelBuilder {
    agent_id: Option<String>,
    hold_audio: Option<AgentHoldAudioConfig>,
}

impl PostAgentHoldAudioResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn hold_audio(mut self, value: AgentHoldAudioConfig) -> Self {
        self.hold_audio = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostAgentHoldAudioResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](PostAgentHoldAudioResponseModelBuilder::agent_id)
    /// - [`hold_audio`](PostAgentHoldAudioResponseModelBuilder::hold_audio)
    pub fn build(self) -> Result<PostAgentHoldAudioResponseModel, BuildError> {
        Ok(PostAgentHoldAudioResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            hold_audio: self.hold_audio.ok_or_else(|| BuildError::missing_field("hold_audio"))?,
        })
    }
}
