pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentConfig {
    /// If non-empty, the first message the agent will say. If empty, the agent waits for the user to start the discussion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_message: Option<String>,
    /// Language of the agent - used for ASR and TTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// When enabled and language is Hindi, the agent will respond in Hinglish
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hinglish_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variables: Option<serde_json::Value>,
    /// If true, the user will not be able to interrupt the agent while the first message is being delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_first_message_interruptions: Option<bool>,
    /// If non-empty, the message the agent will send when max conversation duration is reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conversation_duration_message: Option<String>,
    /// Per-channel response behavior overrides for text conversations. Built-in channel defaults apply when unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_behavior_overrides: Option<HashMap<String, Option<BehaviorOverride>>>,
    /// The prompt for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptAgentApiModelOutput>,
}

impl AgentConfig {
    pub fn builder() -> AgentConfigBuilder {
        <AgentConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentConfigBuilder {
    first_message: Option<String>,
    language: Option<String>,
    hinglish_mode: Option<bool>,
    dynamic_variables: Option<serde_json::Value>,
    disable_first_message_interruptions: Option<bool>,
    max_conversation_duration_message: Option<String>,
    text_behavior_overrides: Option<HashMap<String, Option<BehaviorOverride>>>,
    prompt: Option<PromptAgentApiModelOutput>,
}

impl AgentConfigBuilder {
    pub fn first_message(mut self, value: impl Into<String>) -> Self {
        self.first_message = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn hinglish_mode(mut self, value: bool) -> Self {
        self.hinglish_mode = Some(value);
        self
    }

    pub fn dynamic_variables(mut self, value: serde_json::Value) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    pub fn disable_first_message_interruptions(mut self, value: bool) -> Self {
        self.disable_first_message_interruptions = Some(value);
        self
    }

    pub fn max_conversation_duration_message(mut self, value: impl Into<String>) -> Self {
        self.max_conversation_duration_message = Some(value.into());
        self
    }

    pub fn text_behavior_overrides(mut self, value: HashMap<String, Option<BehaviorOverride>>) -> Self {
        self.text_behavior_overrides = Some(value);
        self
    }

    pub fn prompt(mut self, value: PromptAgentApiModelOutput) -> Self {
        self.prompt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentConfig`].
    pub fn build(self) -> Result<AgentConfig, BuildError> {
        Ok(AgentConfig {
            first_message: self.first_message,
            language: self.language,
            hinglish_mode: self.hinglish_mode,
            dynamic_variables: self.dynamic_variables,
            disable_first_message_interruptions: self.disable_first_message_interruptions,
            max_conversation_duration_message: self.max_conversation_duration_message,
            text_behavior_overrides: self.text_behavior_overrides,
            prompt: self.prompt,
        })
    }
}
