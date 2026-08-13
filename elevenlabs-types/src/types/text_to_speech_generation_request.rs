pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "model_id")]
#[non_exhaustive]
pub enum TextToSpeechGenerationRequest {
        #[serde(rename = "eleven_flash_v2_5")]
        #[non_exhaustive]
        ElevenFlashV25 {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            text: String,
            #[serde(default)]
            voice: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            output_format: Option<ElevenFlashV25RequestOutputFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            language_code: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            voice_settings: Option<ElevenFlashV25VoiceSettings>,
        },

        #[serde(rename = "eleven_multilingual_v2")]
        #[non_exhaustive]
        ElevenMultilingualV2 {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            text: String,
            #[serde(default)]
            voice: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            output_format: Option<ElevenMultilingualV2RequestOutputFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            voice_settings: Option<TtsVoiceSettings>,
        },

        #[serde(rename = "eleven_v3")]
        #[non_exhaustive]
        ElevenV3 {
            #[serde(skip_serializing_if = "Option::is_none")]
            webhook: Option<WebhookTarget>,
            #[serde(default)]
            text: String,
            #[serde(default)]
            voice: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            output_format: Option<ElevenV3RequestOutputFormat>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            language_code: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            voice_settings: Option<ElevenV3VoiceSettings>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl TextToSpeechGenerationRequest {
    pub fn eleven_flash_v25(text: String, voice: String) -> Self {
        Self::ElevenFlashV25 { webhook: None, text, voice, output_format: None, pronunciation_dictionary_locators: None, language_code: None, voice_settings: None }
    }

    pub fn eleven_multilingual_v2(text: String, voice: String) -> Self {
        Self::ElevenMultilingualV2 { webhook: None, text, voice, output_format: None, pronunciation_dictionary_locators: None, voice_settings: None }
    }

    pub fn eleven_v3(text: String, voice: String) -> Self {
        Self::ElevenV3 { webhook: None, text, voice, output_format: None, pronunciation_dictionary_locators: None, language_code: None, voice_settings: None }
    }

    pub fn eleven_flash_v25_with_webhook(webhook: WebhookTarget, text: String, voice: String, output_format: Option<ElevenFlashV25RequestOutputFormat>, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, language_code: Option<String>, voice_settings: Option<ElevenFlashV25VoiceSettings>) -> Self {
        Self::ElevenFlashV25 { webhook: Some(webhook), text, voice, output_format, pronunciation_dictionary_locators, language_code, voice_settings }
    }

    pub fn eleven_flash_v25_with_output_format(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: ElevenFlashV25RequestOutputFormat, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, language_code: Option<String>, voice_settings: Option<ElevenFlashV25VoiceSettings>) -> Self {
        Self::ElevenFlashV25 { webhook, text, voice, output_format: Some(output_format), pronunciation_dictionary_locators, language_code, voice_settings }
    }

    pub fn eleven_flash_v25_with_pronunciation_dictionary_locators(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: Option<ElevenFlashV25RequestOutputFormat>, pronunciation_dictionary_locators: Vec<PronunciationDictionaryVersionLocator>, language_code: Option<String>, voice_settings: Option<ElevenFlashV25VoiceSettings>) -> Self {
        Self::ElevenFlashV25 { webhook, text, voice, output_format, pronunciation_dictionary_locators: Some(pronunciation_dictionary_locators), language_code, voice_settings }
    }

    pub fn eleven_flash_v25_with_language_code(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: Option<ElevenFlashV25RequestOutputFormat>, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, language_code: String, voice_settings: Option<ElevenFlashV25VoiceSettings>) -> Self {
        Self::ElevenFlashV25 { webhook, text, voice, output_format, pronunciation_dictionary_locators, language_code: Some(language_code), voice_settings }
    }

    pub fn eleven_flash_v25_with_voice_settings(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: Option<ElevenFlashV25RequestOutputFormat>, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, language_code: Option<String>, voice_settings: ElevenFlashV25VoiceSettings) -> Self {
        Self::ElevenFlashV25 { webhook, text, voice, output_format, pronunciation_dictionary_locators, language_code, voice_settings: Some(voice_settings) }
    }

    pub fn eleven_multilingual_v2_with_webhook(webhook: WebhookTarget, text: String, voice: String, output_format: Option<ElevenMultilingualV2RequestOutputFormat>, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, voice_settings: Option<TtsVoiceSettings>) -> Self {
        Self::ElevenMultilingualV2 { webhook: Some(webhook), text, voice, output_format, pronunciation_dictionary_locators, voice_settings }
    }

    pub fn eleven_multilingual_v2_with_output_format(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: ElevenMultilingualV2RequestOutputFormat, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, voice_settings: Option<TtsVoiceSettings>) -> Self {
        Self::ElevenMultilingualV2 { webhook, text, voice, output_format: Some(output_format), pronunciation_dictionary_locators, voice_settings }
    }

    pub fn eleven_multilingual_v2_with_pronunciation_dictionary_locators(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: Option<ElevenMultilingualV2RequestOutputFormat>, pronunciation_dictionary_locators: Vec<PronunciationDictionaryVersionLocator>, voice_settings: Option<TtsVoiceSettings>) -> Self {
        Self::ElevenMultilingualV2 { webhook, text, voice, output_format, pronunciation_dictionary_locators: Some(pronunciation_dictionary_locators), voice_settings }
    }

    pub fn eleven_multilingual_v2_with_voice_settings(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: Option<ElevenMultilingualV2RequestOutputFormat>, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, voice_settings: TtsVoiceSettings) -> Self {
        Self::ElevenMultilingualV2 { webhook, text, voice, output_format, pronunciation_dictionary_locators, voice_settings: Some(voice_settings) }
    }

    pub fn eleven_v3_with_webhook(webhook: WebhookTarget, text: String, voice: String, output_format: Option<ElevenV3RequestOutputFormat>, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, language_code: Option<String>, voice_settings: Option<ElevenV3VoiceSettings>) -> Self {
        Self::ElevenV3 { webhook: Some(webhook), text, voice, output_format, pronunciation_dictionary_locators, language_code, voice_settings }
    }

    pub fn eleven_v3_with_output_format(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: ElevenV3RequestOutputFormat, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, language_code: Option<String>, voice_settings: Option<ElevenV3VoiceSettings>) -> Self {
        Self::ElevenV3 { webhook, text, voice, output_format: Some(output_format), pronunciation_dictionary_locators, language_code, voice_settings }
    }

    pub fn eleven_v3_with_pronunciation_dictionary_locators(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: Option<ElevenV3RequestOutputFormat>, pronunciation_dictionary_locators: Vec<PronunciationDictionaryVersionLocator>, language_code: Option<String>, voice_settings: Option<ElevenV3VoiceSettings>) -> Self {
        Self::ElevenV3 { webhook, text, voice, output_format, pronunciation_dictionary_locators: Some(pronunciation_dictionary_locators), language_code, voice_settings }
    }

    pub fn eleven_v3_with_language_code(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: Option<ElevenV3RequestOutputFormat>, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, language_code: String, voice_settings: Option<ElevenV3VoiceSettings>) -> Self {
        Self::ElevenV3 { webhook, text, voice, output_format, pronunciation_dictionary_locators, language_code: Some(language_code), voice_settings }
    }

    pub fn eleven_v3_with_voice_settings(webhook: Option<WebhookTarget>, text: String, voice: String, output_format: Option<ElevenV3RequestOutputFormat>, pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>, language_code: Option<String>, voice_settings: ElevenV3VoiceSettings) -> Self {
        Self::ElevenV3 { webhook, text, voice, output_format, pronunciation_dictionary_locators, language_code, voice_settings: Some(voice_settings) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
