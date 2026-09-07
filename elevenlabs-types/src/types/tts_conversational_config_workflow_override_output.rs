pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TtsConversationalConfigWorkflowOverrideOutput {
    /// The model to use for TTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<TtsConversationalModel>,
    /// The voice ID to use for TTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// Additional supported voices for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_voices: Option<Vec<SupportedVoice>>,
    /// When enabled, applies expressive audio tags prompt. Automatically disabled for non-v3 models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expressive_mode: Option<bool>,
    /// Suggested audio tags to boost expressive speech (for eleven_v3 and eleven_v3_conversational models). The agent can still use other tags not listed here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_audio_tags: Option<Vec<SuggestedAudioTag>>,
    /// The audio format to use for TTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_output_audio_format: Option<TtsOutputFormat>,
    /// Deprecated: this field is a no-op and is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_streaming_latency: Option<TtsOptimizeStreamingLatency>,
    /// The stability of generated speech
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
    /// The speed of generated speech
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub speed: Option<f64>,
    /// The similarity boost for generated speech
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub similarity_boost: Option<f64>,
    /// Method for converting numbers to words before converting text to speech. If set to SYSTEM_PROMPT, the system prompt will be updated to include normalization instructions. If set to ELEVENLABS, the text will be normalized after generation, incurring slight additional latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_normalisation_type: Option<TextNormalisationType>,
    /// The pronunciation dictionary locators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PydanticPronunciationDictionaryVersionLocator>>,
    /// Opt-in to SSML phoneme tag handling for V3 models. When enabled, phoneme tags (inline and from pronunciation dictionaries) are parsed into inline IPA before being sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_phoneme_tags: Option<bool>,
    /// Optional TTS effects spec: filter preset, distance (proximity EQ), and environment (convolution reverb).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_effects: Option<EffectsSpecOutput>,
}

impl TtsConversationalConfigWorkflowOverrideOutput {
    pub fn builder() -> TtsConversationalConfigWorkflowOverrideOutputBuilder {
        <TtsConversationalConfigWorkflowOverrideOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TtsConversationalConfigWorkflowOverrideOutputBuilder {
    model_id: Option<TtsConversationalModel>,
    voice_id: Option<String>,
    supported_voices: Option<Vec<SupportedVoice>>,
    expressive_mode: Option<bool>,
    suggested_audio_tags: Option<Vec<SuggestedAudioTag>>,
    agent_output_audio_format: Option<TtsOutputFormat>,
    optimize_streaming_latency: Option<TtsOptimizeStreamingLatency>,
    stability: Option<f64>,
    speed: Option<f64>,
    similarity_boost: Option<f64>,
    text_normalisation_type: Option<TextNormalisationType>,
    pronunciation_dictionary_locators: Option<Vec<PydanticPronunciationDictionaryVersionLocator>>,
    enable_phoneme_tags: Option<bool>,
    audio_effects: Option<EffectsSpecOutput>,
}

impl TtsConversationalConfigWorkflowOverrideOutputBuilder {
    pub fn model_id(mut self, value: TtsConversationalModel) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn supported_voices(mut self, value: Vec<SupportedVoice>) -> Self {
        self.supported_voices = Some(value);
        self
    }

    pub fn expressive_mode(mut self, value: bool) -> Self {
        self.expressive_mode = Some(value);
        self
    }

    pub fn suggested_audio_tags(mut self, value: Vec<SuggestedAudioTag>) -> Self {
        self.suggested_audio_tags = Some(value);
        self
    }

    pub fn agent_output_audio_format(mut self, value: TtsOutputFormat) -> Self {
        self.agent_output_audio_format = Some(value);
        self
    }

    pub fn optimize_streaming_latency(mut self, value: TtsOptimizeStreamingLatency) -> Self {
        self.optimize_streaming_latency = Some(value);
        self
    }

    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    pub fn speed(mut self, value: f64) -> Self {
        self.speed = Some(value);
        self
    }

    pub fn similarity_boost(mut self, value: f64) -> Self {
        self.similarity_boost = Some(value);
        self
    }

    pub fn text_normalisation_type(mut self, value: TextNormalisationType) -> Self {
        self.text_normalisation_type = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<PydanticPronunciationDictionaryVersionLocator>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn enable_phoneme_tags(mut self, value: bool) -> Self {
        self.enable_phoneme_tags = Some(value);
        self
    }

    pub fn audio_effects(mut self, value: EffectsSpecOutput) -> Self {
        self.audio_effects = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TtsConversationalConfigWorkflowOverrideOutput`].
    pub fn build(self) -> Result<TtsConversationalConfigWorkflowOverrideOutput, BuildError> {
        Ok(TtsConversationalConfigWorkflowOverrideOutput {
            model_id: self.model_id,
            voice_id: self.voice_id,
            supported_voices: self.supported_voices,
            expressive_mode: self.expressive_mode,
            suggested_audio_tags: self.suggested_audio_tags,
            agent_output_audio_format: self.agent_output_audio_format,
            optimize_streaming_latency: self.optimize_streaming_latency,
            stability: self.stability,
            speed: self.speed,
            similarity_boost: self.similarity_boost,
            text_normalisation_type: self.text_normalisation_type,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            enable_phoneme_tags: self.enable_phoneme_tags,
            audio_effects: self.audio_effects,
        })
    }
}
