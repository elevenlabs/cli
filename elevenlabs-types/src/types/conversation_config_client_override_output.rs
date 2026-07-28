pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationConfigClientOverrideOutput {
    /// Configuration for conversational transcription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr: Option<AsrConversationalConfigOverride>,
    /// Configuration for turn detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<TurnConfigOverride>,
    /// Configuration for conversational text to speech
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<TtsConversationalConfigOverride>,
    /// Configuration for conversational events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationConfigOverride>,
    /// Agent specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentConfigOverrideOutput>,
}

impl ConversationConfigClientOverrideOutput {
    pub fn builder() -> ConversationConfigClientOverrideOutputBuilder {
        <ConversationConfigClientOverrideOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationConfigClientOverrideOutputBuilder {
    asr: Option<AsrConversationalConfigOverride>,
    turn: Option<TurnConfigOverride>,
    tts: Option<TtsConversationalConfigOverride>,
    conversation: Option<ConversationConfigOverride>,
    agent: Option<AgentConfigOverrideOutput>,
}

impl ConversationConfigClientOverrideOutputBuilder {
    pub fn asr(mut self, value: AsrConversationalConfigOverride) -> Self {
        self.asr = Some(value);
        self
    }

    pub fn turn(mut self, value: TurnConfigOverride) -> Self {
        self.turn = Some(value);
        self
    }

    pub fn tts(mut self, value: TtsConversationalConfigOverride) -> Self {
        self.tts = Some(value);
        self
    }

    pub fn conversation(mut self, value: ConversationConfigOverride) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn agent(mut self, value: AgentConfigOverrideOutput) -> Self {
        self.agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationConfigClientOverrideOutput`].
    pub fn build(self) -> Result<ConversationConfigClientOverrideOutput, BuildError> {
        Ok(ConversationConfigClientOverrideOutput {
            asr: self.asr,
            turn: self.turn,
            tts: self.tts,
            conversation: self.conversation,
            agent: self.agent,
        })
    }
}
