pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeechEngineResponse {
    /// The speech engine resource ID
    #[serde(default)]
    pub speech_engine_id: String,
    /// Human-readable name for the speech engine
    #[serde(default)]
    pub name: String,
    /// WebSocket connection settings for the upstream transcript server
    #[serde(default)]
    pub speech_engine: SpeechEngineConfig,
    /// Automatic speech recognition configuration
    #[serde(default)]
    pub asr: AsrConversationalConfig,
    /// Text-to-speech output configuration
    #[serde(default)]
    pub tts: TtsConversationalConfigOutput,
    /// Turn detection configuration
    #[serde(default)]
    pub turn: BaseTurnConfig,
    /// Conversation-level settings including client events and duration limits
    #[serde(default)]
    pub conversation: ConversationConfigOutput,
    /// Privacy settings controlling recording, retention, and PII handling
    #[serde(default)]
    pub privacy: PrivacyConfigOutput,
    /// Concurrency and daily conversation limits for this speech engine
    #[serde(default)]
    pub call_limits: AgentCallLimits,
    /// ISO language code used by the speech engine (e.g. 'en')
    #[serde(default)]
    pub language: String,
    /// Arbitrary tags for categorization and filtering
    #[serde(default)]
    pub tags: Vec<String>,
    /// Override settings the client may set during conversation initiation
    #[serde(default)]
    pub overrides: SpeechEngineConversationInitiationClientDataConfig,
    /// Creation and update timestamps with source information
    #[serde(default)]
    pub metadata: AgentMetadataDbModel,
    /// The access information of the speech engine for the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_info: Option<ResourceAccessInfo>,
}

impl SpeechEngineResponse {
    pub fn builder() -> SpeechEngineResponseBuilder {
        <SpeechEngineResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechEngineResponseBuilder {
    speech_engine_id: Option<String>,
    name: Option<String>,
    speech_engine: Option<SpeechEngineConfig>,
    asr: Option<AsrConversationalConfig>,
    tts: Option<TtsConversationalConfigOutput>,
    turn: Option<BaseTurnConfig>,
    conversation: Option<ConversationConfigOutput>,
    privacy: Option<PrivacyConfigOutput>,
    call_limits: Option<AgentCallLimits>,
    language: Option<String>,
    tags: Option<Vec<String>>,
    overrides: Option<SpeechEngineConversationInitiationClientDataConfig>,
    metadata: Option<AgentMetadataDbModel>,
    access_info: Option<ResourceAccessInfo>,
}

impl SpeechEngineResponseBuilder {
    pub fn speech_engine_id(mut self, value: impl Into<String>) -> Self {
        self.speech_engine_id = Some(value.into());
        self
    }

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

    pub fn tts(mut self, value: TtsConversationalConfigOutput) -> Self {
        self.tts = Some(value);
        self
    }

    pub fn turn(mut self, value: BaseTurnConfig) -> Self {
        self.turn = Some(value);
        self
    }

    pub fn conversation(mut self, value: ConversationConfigOutput) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn privacy(mut self, value: PrivacyConfigOutput) -> Self {
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

    pub fn metadata(mut self, value: AgentMetadataDbModel) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeechEngineResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`speech_engine_id`](SpeechEngineResponseBuilder::speech_engine_id)
    /// - [`name`](SpeechEngineResponseBuilder::name)
    /// - [`speech_engine`](SpeechEngineResponseBuilder::speech_engine)
    /// - [`asr`](SpeechEngineResponseBuilder::asr)
    /// - [`tts`](SpeechEngineResponseBuilder::tts)
    /// - [`turn`](SpeechEngineResponseBuilder::turn)
    /// - [`conversation`](SpeechEngineResponseBuilder::conversation)
    /// - [`privacy`](SpeechEngineResponseBuilder::privacy)
    /// - [`call_limits`](SpeechEngineResponseBuilder::call_limits)
    /// - [`language`](SpeechEngineResponseBuilder::language)
    /// - [`tags`](SpeechEngineResponseBuilder::tags)
    /// - [`overrides`](SpeechEngineResponseBuilder::overrides)
    /// - [`metadata`](SpeechEngineResponseBuilder::metadata)
    pub fn build(self) -> Result<SpeechEngineResponse, BuildError> {
        Ok(SpeechEngineResponse {
            speech_engine_id: self.speech_engine_id.ok_or_else(|| BuildError::missing_field("speech_engine_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            speech_engine: self.speech_engine.ok_or_else(|| BuildError::missing_field("speech_engine"))?,
            asr: self.asr.ok_or_else(|| BuildError::missing_field("asr"))?,
            tts: self.tts.ok_or_else(|| BuildError::missing_field("tts"))?,
            turn: self.turn.ok_or_else(|| BuildError::missing_field("turn"))?,
            conversation: self.conversation.ok_or_else(|| BuildError::missing_field("conversation"))?,
            privacy: self.privacy.ok_or_else(|| BuildError::missing_field("privacy"))?,
            call_limits: self.call_limits.ok_or_else(|| BuildError::missing_field("call_limits"))?,
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
            overrides: self.overrides.ok_or_else(|| BuildError::missing_field("overrides"))?,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            access_info: self.access_info,
        })
    }
}
