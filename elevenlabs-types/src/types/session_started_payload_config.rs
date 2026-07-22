pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Configuration for the transcription session.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionStartedPayloadConfig {
    /// Sample rate of the audio in Hz.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_format: Option<AudioFormatEnum>,
    /// Language code in ISO 639-1 or ISO 639-3 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Strategy for committing transcriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_strategy: Option<SessionStartedPayloadConfigCommitStrategy>,
    /// Silence threshold in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub vad_silence_threshold_secs: Option<f64>,
    /// Threshold for voice activity detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub vad_threshold: Option<f64>,
    /// Minimum speech duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_speech_duration_ms: Option<i64>,
    /// Minimum silence duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_silence_duration_ms: Option<i64>,
    /// ID of the model to use for transcription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request. Zero retention mode may only be used by enterprise customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_logging: Option<bool>,
    /// Whether the session will include word-level timestamps in the committed transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_timestamps: Option<bool>,
    /// Whether the session will include language detection in the committed transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_language_detection: Option<bool>,
    /// List of keyterms the model is biased towards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyterms: Option<Vec<String>>,
    /// Whether filler words and disfluencies are removed from the transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_verbatim: Option<bool>,
}

impl SessionStartedPayloadConfig {
    pub fn builder() -> SessionStartedPayloadConfigBuilder {
        <SessionStartedPayloadConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionStartedPayloadConfigBuilder {
    sample_rate: Option<i64>,
    audio_format: Option<AudioFormatEnum>,
    language_code: Option<String>,
    commit_strategy: Option<SessionStartedPayloadConfigCommitStrategy>,
    vad_silence_threshold_secs: Option<f64>,
    vad_threshold: Option<f64>,
    min_speech_duration_ms: Option<i64>,
    min_silence_duration_ms: Option<i64>,
    model_id: Option<String>,
    enable_logging: Option<bool>,
    include_timestamps: Option<bool>,
    include_language_detection: Option<bool>,
    keyterms: Option<Vec<String>>,
    no_verbatim: Option<bool>,
}

impl SessionStartedPayloadConfigBuilder {
    pub fn sample_rate(mut self, value: i64) -> Self {
        self.sample_rate = Some(value);
        self
    }

    pub fn audio_format(mut self, value: AudioFormatEnum) -> Self {
        self.audio_format = Some(value);
        self
    }

    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn commit_strategy(mut self, value: SessionStartedPayloadConfigCommitStrategy) -> Self {
        self.commit_strategy = Some(value);
        self
    }

    pub fn vad_silence_threshold_secs(mut self, value: f64) -> Self {
        self.vad_silence_threshold_secs = Some(value);
        self
    }

    pub fn vad_threshold(mut self, value: f64) -> Self {
        self.vad_threshold = Some(value);
        self
    }

    pub fn min_speech_duration_ms(mut self, value: i64) -> Self {
        self.min_speech_duration_ms = Some(value);
        self
    }

    pub fn min_silence_duration_ms(mut self, value: i64) -> Self {
        self.min_silence_duration_ms = Some(value);
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    pub fn include_timestamps(mut self, value: bool) -> Self {
        self.include_timestamps = Some(value);
        self
    }

    pub fn include_language_detection(mut self, value: bool) -> Self {
        self.include_language_detection = Some(value);
        self
    }

    pub fn keyterms(mut self, value: Vec<String>) -> Self {
        self.keyterms = Some(value);
        self
    }

    pub fn no_verbatim(mut self, value: bool) -> Self {
        self.no_verbatim = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionStartedPayloadConfig`].
    pub fn build(self) -> Result<SessionStartedPayloadConfig, BuildError> {
        Ok(SessionStartedPayloadConfig {
            sample_rate: self.sample_rate,
            audio_format: self.audio_format,
            language_code: self.language_code,
            commit_strategy: self.commit_strategy,
            vad_silence_threshold_secs: self.vad_silence_threshold_secs,
            vad_threshold: self.vad_threshold,
            min_speech_duration_ms: self.min_speech_duration_ms,
            min_silence_duration_ms: self.min_silence_duration_ms,
            model_id: self.model_id,
            enable_logging: self.enable_logging,
            include_timestamps: self.include_timestamps,
            include_language_detection: self.include_language_detection,
            keyterms: self.keyterms,
            no_verbatim: self.no_verbatim,
        })
    }
}
