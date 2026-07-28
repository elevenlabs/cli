pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamTextToSpeechRequest {
    /// The text that will get converted into speech.
    #[serde(default)]
    pub text: String,
    /// Identifier of the model that will be used, you can query them using GET /v1/models. The model needs to have support for text to speech, you can check this using the can_do_text_to_speech property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// Language code (ISO 639-1) used to enforce a language for the model and text normalization. If the model does not support the provided language code, it will be ignored. This parameter is not supported for multilingual_v2 models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Voice settings overriding stored settings for the given voice. They are applied only on the given request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
    /// A list of pronunciation dictionary locators (id, version_id) to be applied to the text. They will be applied in order. You may have up to 3 locators per request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>,
    /// If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same seed and parameters should return the same result. Determinism is not guaranteed. Must be integer between 0 and 4294967295.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// The text that came before the text of the current request. Can be used to improve the speech's continuity when concatenating together multiple generations or to influence the speech's continuity in the current generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_text: Option<String>,
    /// The text that comes after the text of the current request. Can be used to improve the speech's continuity when concatenating together multiple generations or to influence the speech's continuity in the current generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_text: Option<String>,
    /// A list of request_id of the samples that were generated before this generation. Can be used to improve the speech's continuity when splitting up a large task into multiple requests. The results will be best when the same model is used across the generations. In case both previous_text and previous_request_ids is send, previous_text will be ignored. A maximum of 3 request_ids can be send.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_request_ids: Option<Vec<String>>,
    /// A list of request_id of the samples that come after this generation. next_request_ids is especially useful for maintaining the speech's continuity when regenerating a sample that has had some audio quality issues. For example, if you have generated 3 speech clips, and you want to improve clip 2, passing the request id of clip 3 as a next_request_id (and that of clip 1 as a previous_request_id) will help maintain natural flow in the combined speech. The results will be best when the same model is used across the generations. In case both next_text and next_request_ids is send, next_text will be ignored. A maximum of 3 request_ids can be send.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_request_ids: Option<Vec<String>>,
    /// If true, we won't use PVC version of the voice for the generation but the IVC version. This is a temporary workaround for higher latency in PVC versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_pvc_as_ivc: Option<bool>,
    /// This parameter controls text normalization with three modes: 'auto', 'on', and 'off'. When set to 'auto', the system will automatically decide whether to apply text normalization (e.g., spelling out numbers). With 'on', text normalization will always be applied, while with 'off', it will be skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<BodyTextToSpeechStreamApplyTextNormalization>,
    /// This parameter controls language text normalization. This helps with proper pronunciation of text in some supported languages. WARNING: This parameter can heavily increase the latency of the request. Currently only supported for Japanese.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_language_text_normalization: Option<bool>,
    /// When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    #[serde(skip)]
    pub enable_logging: Option<bool>,
    /// You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values:
    /// 0 - default mode (no latency optimizations)
    /// 1 - normal latency optimizations (about 50% of possible latency improvement of option 3)
    /// 2 - strong latency optimizations (about 75% of possible latency improvement of option 3)
    /// 3 - max latency optimizations
    /// 4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).
    ///
    /// Defaults to None.
    #[serde(skip)]
    pub optimize_streaming_latency: Option<i64>,
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    #[serde(skip)]
    pub output_format: Option<TextToSpeechStreamRequestOutputFormat>,
}

impl StreamTextToSpeechRequest {
    pub fn builder() -> StreamTextToSpeechRequestBuilder {
        <StreamTextToSpeechRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamTextToSpeechRequestBuilder {
    text: Option<String>,
    model_id: Option<String>,
    language_code: Option<String>,
    voice_settings: Option<VoiceSettings>,
    pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>,
    seed: Option<i64>,
    previous_text: Option<String>,
    next_text: Option<String>,
    previous_request_ids: Option<Vec<String>>,
    next_request_ids: Option<Vec<String>>,
    use_pvc_as_ivc: Option<bool>,
    apply_text_normalization: Option<BodyTextToSpeechStreamApplyTextNormalization>,
    apply_language_text_normalization: Option<bool>,
    enable_logging: Option<bool>,
    optimize_streaming_latency: Option<i64>,
    output_format: Option<TextToSpeechStreamRequestOutputFormat>,
}

impl StreamTextToSpeechRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
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

    pub fn voice_settings(mut self, value: VoiceSettings) -> Self {
        self.voice_settings = Some(value);
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

    pub fn previous_text(mut self, value: impl Into<String>) -> Self {
        self.previous_text = Some(value.into());
        self
    }

    pub fn next_text(mut self, value: impl Into<String>) -> Self {
        self.next_text = Some(value.into());
        self
    }

    pub fn previous_request_ids(mut self, value: Vec<String>) -> Self {
        self.previous_request_ids = Some(value);
        self
    }

    pub fn next_request_ids(mut self, value: Vec<String>) -> Self {
        self.next_request_ids = Some(value);
        self
    }

    pub fn use_pvc_as_ivc(mut self, value: bool) -> Self {
        self.use_pvc_as_ivc = Some(value);
        self
    }

    pub fn apply_text_normalization(mut self, value: BodyTextToSpeechStreamApplyTextNormalization) -> Self {
        self.apply_text_normalization = Some(value);
        self
    }

    pub fn apply_language_text_normalization(mut self, value: bool) -> Self {
        self.apply_language_text_normalization = Some(value);
        self
    }

    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    pub fn optimize_streaming_latency(mut self, value: i64) -> Self {
        self.optimize_streaming_latency = Some(value);
        self
    }

    pub fn output_format(mut self, value: TextToSpeechStreamRequestOutputFormat) -> Self {
        self.output_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamTextToSpeechRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](StreamTextToSpeechRequestBuilder::text)
    pub fn build(self) -> Result<StreamTextToSpeechRequest, BuildError> {
        Ok(StreamTextToSpeechRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            model_id: self.model_id,
            language_code: self.language_code,
            voice_settings: self.voice_settings,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            seed: self.seed,
            previous_text: self.previous_text,
            next_text: self.next_text,
            previous_request_ids: self.previous_request_ids,
            next_request_ids: self.next_request_ids,
            use_pvc_as_ivc: self.use_pvc_as_ivc,
            apply_text_normalization: self.apply_text_normalization,
            apply_language_text_normalization: self.apply_language_text_normalization,
            enable_logging: self.enable_logging,
            optimize_streaming_latency: self.optimize_streaming_latency,
            output_format: self.output_format,
        })
    }
}

