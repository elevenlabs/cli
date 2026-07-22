pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyTextToDialogueStreamWithTimestamps {
    /// A list of dialogue inputs, each containing text and a voice ID which will be converted into speech. The maximum number of unique voice IDs is 10. For reliable generation, keep the total character count across all `inputs[].text` values at or below 2,000 characters per request. Longer requests can terminate early in streaming responses or return a validation error.
    #[serde(default)]
    pub inputs: Vec<DialogueInput>,
    /// Identifier of the model that will be used, you can query them using GET /v1/models. The model needs to have support for text to speech, you can check this using the can_do_text_to_speech property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Language code (ISO 639-1) used to enforce a language for the model and text normalization. If the model does not support the provided language code, it will be ignored. This parameter is not supported for multilingual_v2 models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Settings controlling the dialogue generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ModelSettingsResponseModel>,
    /// A list of pronunciation dictionary locators (id, version_id) to be applied to the text. They will be applied in order. You may have up to 3 locators per request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>,
    /// If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result. Determinism is not guaranteed. Must be integer between 0 and 4294967295.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// This parameter controls text normalization with three modes: 'auto', 'on', and 'off'. When set to 'auto', the system will automatically decide whether to apply text normalization (e.g., spelling out numbers). With 'on', text normalization will always be applied, while with 'off', it will be skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<BodyTextToDialogueStreamWithTimestampsApplyTextNormalization>,
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    #[serde(skip)]
    pub output_format: Option<AllowedOutputFormats>,
    /// When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    #[serde(skip)]
    pub enable_logging: Option<bool>,
}

impl BodyTextToDialogueStreamWithTimestamps {
    pub fn builder() -> BodyTextToDialogueStreamWithTimestampsBuilder {
        <BodyTextToDialogueStreamWithTimestampsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyTextToDialogueStreamWithTimestampsBuilder {
    inputs: Option<Vec<DialogueInput>>,
    model_id: Option<String>,
    language_code: Option<String>,
    settings: Option<ModelSettingsResponseModel>,
    pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>,
    seed: Option<i64>,
    apply_text_normalization: Option<BodyTextToDialogueStreamWithTimestampsApplyTextNormalization>,
    output_format: Option<AllowedOutputFormats>,
    enable_logging: Option<bool>,
}

impl BodyTextToDialogueStreamWithTimestampsBuilder {
    pub fn inputs(mut self, value: Vec<DialogueInput>) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn settings(mut self, value: ModelSettingsResponseModel) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<PronunciationDictionaryVersionLocator>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn apply_text_normalization(mut self, value: BodyTextToDialogueStreamWithTimestampsApplyTextNormalization) -> Self {
        self.apply_text_normalization = Some(value);
        self
    }

    pub fn output_format(mut self, value: AllowedOutputFormats) -> Self {
        self.output_format = Some(value);
        self
    }

    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyTextToDialogueStreamWithTimestamps`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inputs`](BodyTextToDialogueStreamWithTimestampsBuilder::inputs)
    pub fn build(self) -> Result<BodyTextToDialogueStreamWithTimestamps, BuildError> {
        Ok(BodyTextToDialogueStreamWithTimestamps {
            inputs: self.inputs.ok_or_else(|| BuildError::missing_field("inputs"))?,
            model_id: self.model_id,
            language_code: self.language_code,
            settings: self.settings,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            seed: self.seed,
            apply_text_normalization: self.apply_text_normalization,
            output_format: self.output_format,
            enable_logging: self.enable_logging,
        })
    }
}

