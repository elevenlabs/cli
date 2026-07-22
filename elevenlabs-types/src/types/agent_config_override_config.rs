pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentConfigOverrideConfig {
    /// Whether to allow overriding the first_message field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_message: Option<bool>,
    /// Whether to allow overriding the language field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<bool>,
    /// Whether to allow overriding the max_conversation_duration_message field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_conversation_duration_message: Option<bool>,
    /// Configures overrides for nested fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptAgentApiModelOverrideConfig>,
}

impl AgentConfigOverrideConfig {
    pub fn builder() -> AgentConfigOverrideConfigBuilder {
        <AgentConfigOverrideConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentConfigOverrideConfigBuilder {
    first_message: Option<bool>,
    language: Option<bool>,
    max_conversation_duration_message: Option<bool>,
    prompt: Option<PromptAgentApiModelOverrideConfig>,
}

impl AgentConfigOverrideConfigBuilder {
    pub fn first_message(mut self, value: bool) -> Self {
        self.first_message = Some(value);
        self
    }

    pub fn language(mut self, value: bool) -> Self {
        self.language = Some(value);
        self
    }

    pub fn max_conversation_duration_message(mut self, value: bool) -> Self {
        self.max_conversation_duration_message = Some(value);
        self
    }

    pub fn prompt(mut self, value: PromptAgentApiModelOverrideConfig) -> Self {
        self.prompt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentConfigOverrideConfig`].
    pub fn build(self) -> Result<AgentConfigOverrideConfig, BuildError> {
        Ok(AgentConfigOverrideConfig {
            first_message: self.first_message,
            language: self.language,
            max_conversation_duration_message: self.max_conversation_duration_message,
            prompt: self.prompt,
        })
    }
}
