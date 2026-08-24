pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateSpeechEngineRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_engine: Option<SpeechEngineConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr: Option<AsrConversationalConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<TtsConversationalConfigInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<BaseTurnConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad: Option<VadConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationConfigInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<PrivacyConfigInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_limits: Option<AgentCallLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<SpeechEngineConversationInitiationClientDataConfig>,
}

impl UpdateSpeechEngineRequest {
    pub fn builder() -> UpdateSpeechEngineRequestBuilder {
        <UpdateSpeechEngineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSpeechEngineRequestBuilder {
    name: Option<String>,
    speech_engine: Option<SpeechEngineConfig>,
    asr: Option<AsrConversationalConfig>,
    tts: Option<TtsConversationalConfigInput>,
    turn: Option<BaseTurnConfig>,
    vad: Option<VadConfig>,
    conversation: Option<ConversationConfigInput>,
    privacy: Option<PrivacyConfigInput>,
    call_limits: Option<AgentCallLimits>,
    language: Option<String>,
    tags: Option<Vec<String>>,
    overrides: Option<SpeechEngineConversationInitiationClientDataConfig>,
}

impl UpdateSpeechEngineRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn speech_engine(mut self, value: SpeechEngineConfig) -> Self {
        self.speech_engine = Some(value);
        self
    }

    pub fn asr(mut self, value: AsrConversationalConfig) -> Self {
        self.asr = Some(value);
        self
    }

    pub fn tts(mut self, value: TtsConversationalConfigInput) -> Self {
        self.tts = Some(value);
        self
    }

    pub fn turn(mut self, value: BaseTurnConfig) -> Self {
        self.turn = Some(value);
        self
    }

    pub fn vad(mut self, value: VadConfig) -> Self {
        self.vad = Some(value);
        self
    }

    pub fn conversation(mut self, value: ConversationConfigInput) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn privacy(mut self, value: PrivacyConfigInput) -> Self {
        self.privacy = Some(value);
        self
    }

    pub fn call_limits(mut self, value: AgentCallLimits) -> Self {
        self.call_limits = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn overrides(mut self, value: SpeechEngineConversationInitiationClientDataConfig) -> Self {
        self.overrides = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSpeechEngineRequest`].
    pub fn build(self) -> Result<UpdateSpeechEngineRequest, BuildError> {
        Ok(UpdateSpeechEngineRequest {
            name: self.name,
            speech_engine: self.speech_engine,
            asr: self.asr,
            tts: self.tts,
            turn: self.turn,
            vad: self.vad,
            conversation: self.conversation,
            privacy: self.privacy,
            call_limits: self.call_limits,
            language: self.language,
            tags: self.tags,
            overrides: self.overrides,
        })
    }
}

