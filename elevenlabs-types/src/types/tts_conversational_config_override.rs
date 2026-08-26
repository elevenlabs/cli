pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TtsConversationalConfigOverride {
    /// The model to use for TTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<TtsConversationalModel>,
    /// The voice ID to use for TTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// Additional supported voices for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_voices: Option<Vec<SupportedVoice>>,
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
    /// The pronunciation dictionary locators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PydanticPronunciationDictionaryVersionLocator>>,
}

impl TtsConversationalConfigOverride {
    pub fn builder() -> TtsConversationalConfigOverrideBuilder {
        <TtsConversationalConfigOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TtsConversationalConfigOverrideBuilder {
    model_id: Option<TtsConversationalModel>,
    voice_id: Option<String>,
    supported_voices: Option<Vec<SupportedVoice>>,
    stability: Option<f64>,
    speed: Option<f64>,
    similarity_boost: Option<f64>,
    pronunciation_dictionary_locators: Option<Vec<PydanticPronunciationDictionaryVersionLocator>>,
}

impl TtsConversationalConfigOverrideBuilder {
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

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<PydanticPronunciationDictionaryVersionLocator>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TtsConversationalConfigOverride`].
    pub fn build(self) -> Result<TtsConversationalConfigOverride, BuildError> {
        Ok(TtsConversationalConfigOverride {
            model_id: self.model_id,
            voice_id: self.voice_id,
            supported_voices: self.supported_voices,
            stability: self.stability,
            speed: self.speed,
            similarity_boost: self.similarity_boost,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
        })
    }
}
