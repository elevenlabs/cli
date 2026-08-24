pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationalConfig {
    /// Configuration for conversational transcription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr: Option<AsrConversationalConfig>,
    /// Configuration for turn detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<TurnConfig>,
    /// Configuration for conversational text to speech
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<TtsConversationalConfigOutput>,
    /// Configuration for conversational events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationConfigOutput>,
    /// Language presets for conversations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_presets: Option<HashMap<String, LanguagePresetOutput>>,
    /// Configuration for voice activity detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad: Option<VadConfig>,
    /// Agent specific configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentConfig>,
}

impl ConversationalConfig {
    pub fn builder() -> ConversationalConfigBuilder {
        <ConversationalConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalConfigBuilder {
    asr: Option<AsrConversationalConfig>,
    turn: Option<TurnConfig>,
    tts: Option<TtsConversationalConfigOutput>,
    conversation: Option<ConversationConfigOutput>,
    language_presets: Option<HashMap<String, LanguagePresetOutput>>,
    vad: Option<VadConfig>,
    agent: Option<AgentConfig>,
}

impl ConversationalConfigBuilder {
    pub fn asr(mut self, value: AsrConversationalConfig) -> Self {
        self.asr = Some(value);
        self
    }

    pub fn turn(mut self, value: TurnConfig) -> Self {
        self.turn = Some(value);
        self
    }

    pub fn tts(mut self, value: TtsConversationalConfigOutput) -> Self {
        self.tts = Some(value);
        self
    }

    pub fn conversation(mut self, value: ConversationConfigOutput) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn language_presets(mut self, value: HashMap<String, LanguagePresetOutput>) -> Self {
        self.language_presets = Some(value);
        self
    }

    pub fn vad(mut self, value: VadConfig) -> Self {
        self.vad = Some(value);
        self
    }

    pub fn agent(mut self, value: AgentConfig) -> Self {
        self.agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalConfig`].
    pub fn build(self) -> Result<ConversationalConfig, BuildError> {
        Ok(ConversationalConfig {
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
