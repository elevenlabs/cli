pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentConfigOverrideInput {
    /// If non-empty, the first message the agent will say. If empty, the agent waits for the user to start the discussion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_message: Option<String>,
    /// Language of the agent - used for ASR and TTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// If non-empty, the message the agent will send when max conversation duration is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conversation_duration_message: Option<String>,
    /// The prompt for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptAgentApiModelOverrideInput>,
}

impl AgentConfigOverrideInput {
    pub fn builder() -> AgentConfigOverrideInputBuilder {
        <AgentConfigOverrideInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentConfigOverrideInputBuilder {
    first_message: Option<String>,
    language: Option<String>,
    max_conversation_duration_message: Option<String>,
    prompt: Option<PromptAgentApiModelOverrideInput>,
}

impl AgentConfigOverrideInputBuilder {
    pub fn first_message(mut self, value: impl Into<String>) -> Self {
        self.first_message = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn max_conversation_duration_message(mut self, value: impl Into<String>) -> Self {
        self.max_conversation_duration_message = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: PromptAgentApiModelOverrideInput) -> Self {
        self.prompt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentConfigOverrideInput`].
    pub fn build(self) -> Result<AgentConfigOverrideInput, BuildError> {
        Ok(AgentConfigOverrideInput {
            first_message: self.first_message,
            language: self.language,
            max_conversation_duration_message: self.max_conversation_duration_message,
            prompt: self.prompt,
        })
    }
}
