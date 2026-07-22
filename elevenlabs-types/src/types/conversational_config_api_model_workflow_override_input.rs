pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationalConfigApiModelWorkflowOverrideInput {
    /// Configuration for conversational transcription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr: Option<AsrConversationalConfigWorkflowOverride>,
    /// Configuration for turn detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<TurnConfigWorkflowOverride>,
    /// Configuration for conversational text to speech
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<TtsConversationalConfigWorkflowOverrideInput>,
    /// Configuration for conversational events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationConfigWorkflowOverrideInput>,
    /// Language presets for conversations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_presets: Option<HashMap<String, Option<LanguagePresetInput>>>,
    /// Configuration for voice activity detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad: Option<VadConfigWorkflowOverride>,
    /// Agent specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentConfigApiModelWorkflowOverrideInput>,
}

impl ConversationalConfigApiModelWorkflowOverrideInput {
    pub fn builder() -> ConversationalConfigApiModelWorkflowOverrideInputBuilder {
        <ConversationalConfigApiModelWorkflowOverrideInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalConfigApiModelWorkflowOverrideInputBuilder {
    asr: Option<AsrConversationalConfigWorkflowOverride>,
    turn: Option<TurnConfigWorkflowOverride>,
    tts: Option<TtsConversationalConfigWorkflowOverrideInput>,
    conversation: Option<ConversationConfigWorkflowOverrideInput>,
    language_presets: Option<HashMap<String, Option<LanguagePresetInput>>>,
    vad: Option<VadConfigWorkflowOverride>,
    agent: Option<AgentConfigApiModelWorkflowOverrideInput>,
}

impl ConversationalConfigApiModelWorkflowOverrideInputBuilder {
    pub fn asr(mut self, value: AsrConversationalConfigWorkflowOverride) -> Self {
        self.asr = Some(value);
        self
    }

    pub fn turn(mut self, value: TurnConfigWorkflowOverride) -> Self {
        self.turn = Some(value);
        self
    }

    pub fn tts(mut self, value: TtsConversationalConfigWorkflowOverrideInput) -> Self {
        self.tts = Some(value);
        self
    }

    pub fn conversation(mut self, value: ConversationConfigWorkflowOverrideInput) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn language_presets(mut self, value: HashMap<String, Option<LanguagePresetInput>>) -> Self {
        self.language_presets = Some(value);
        self
    }

    pub fn vad(mut self, value: VadConfigWorkflowOverride) -> Self {
        self.vad = Some(value);
        self
    }

    pub fn agent(mut self, value: AgentConfigApiModelWorkflowOverrideInput) -> Self {
        self.agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalConfigApiModelWorkflowOverrideInput`].
    pub fn build(self) -> Result<ConversationalConfigApiModelWorkflowOverrideInput, BuildError> {
        Ok(ConversationalConfigApiModelWorkflowOverrideInput {
            asr: self.asr,
            turn: self.turn,
            tts: self.tts,
            conversation: self.conversation,
            language_presets: self.language_presets,
            vad: self.vad,
            agent: self.agent,
        })
    }
}
