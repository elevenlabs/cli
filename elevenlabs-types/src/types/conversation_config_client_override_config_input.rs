pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationConfigClientOverrideConfigInput {
    /// Configures overrides for nested fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr: Option<AsrConversationalConfigOverrideConfig>,
    /// Configures overrides for nested fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<TurnConfigOverrideConfig>,
    /// Configures overrides for nested fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<TtsConversationalConfigOverrideConfig>,
    /// Configures overrides for nested fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationConfigOverrideConfig>,
    /// Configures overrides for nested fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentConfigOverrideConfig>,
}

impl ConversationConfigClientOverrideConfigInput {
    pub fn builder() -> ConversationConfigClientOverrideConfigInputBuilder {
        <ConversationConfigClientOverrideConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationConfigClientOverrideConfigInputBuilder {
    asr: Option<AsrConversationalConfigOverrideConfig>,
    turn: Option<TurnConfigOverrideConfig>,
    tts: Option<TtsConversationalConfigOverrideConfig>,
    conversation: Option<ConversationConfigOverrideConfig>,
    agent: Option<AgentConfigOverrideConfig>,
}

impl ConversationConfigClientOverrideConfigInputBuilder {
    pub fn asr(mut self, value: AsrConversationalConfigOverrideConfig) -> Self {
        self.asr = Some(value);
        self
    }

    pub fn turn(mut self, value: TurnConfigOverrideConfig) -> Self {
        self.turn = Some(value);
        self
    }

    pub fn tts(mut self, value: TtsConversationalConfigOverrideConfig) -> Self {
        self.tts = Some(value);
        self
    }

    pub fn conversation(mut self, value: ConversationConfigOverrideConfig) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn agent(mut self, value: AgentConfigOverrideConfig) -> Self {
        self.agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationConfigClientOverrideConfigInput`].
    pub fn build(self) -> Result<ConversationConfigClientOverrideConfigInput, BuildError> {
        Ok(ConversationConfigClientOverrideConfigInput {
            asr: self.asr,
            turn: self.turn,
            tts: self.tts,
            conversation: self.conversation,
            agent: self.agent,
        })
    }
}
