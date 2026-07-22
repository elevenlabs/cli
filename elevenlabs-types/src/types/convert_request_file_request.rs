pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvertRequest3 {
    pub model_id: SpeechToTextConvertRequestModelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_audio_events: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_speakers: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps_granularity: Option<SpeechToTextConvertRequestTimestampsGranularity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarize: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub diarization_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_formats: Option<AdditionalFormats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<SpeechToTextConvertRequestFileFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_storage_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_multi_channel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multichannel_output_style: Option<SpeechToTextConvertRequestMultichannelOutputStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_metadata: Option<SpeechToTextConvertRequestWebhookMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_detection: Option<SpeechToTextConvertRequestEntityDetection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_verbatim: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_library: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_speaker_roles: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_redaction: Option<SpeechToTextConvertRequestEntityRedaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_redaction_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyterms: Option<Vec<String>>,
    #[serde(skip)]
    pub token: Option<String>,
    #[serde(skip)]
    pub enable_logging: Option<bool>,
}
impl ConvertRequest3 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    if let Some(ref file_data) = self.file {
        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("file")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Ok(json_str) = serde_json::to_string(&self.model_id) {
        form = form.text("model_id", json_str);
    }

    if let Some(ref value) = self.language_code {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("language_code", json_str);
        }
    }

    if let Some(ref value) = self.tag_audio_events {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("tag_audio_events", json_str);
        }
    }

    if let Some(ref value) = self.num_speakers {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("num_speakers", json_str);
        }
    }

    if let Some(ref value) = self.timestamps_granularity {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("timestamps_granularity", json_str);
        }
    }

    if let Some(ref value) = self.diarize {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("diarize", json_str);
        }
    }

    if let Some(ref value) = self.diarization_threshold {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("diarization_threshold", json_str);
        }
    }

    if let Some(ref value) = self.additional_formats {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("additional_formats", json_str);
        }
    }

    if let Some(ref value) = self.file_format {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("file_format", json_str);
        }
    }

    if let Some(ref value) = self.cloud_storage_url {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("cloud_storage_url", json_str);
        }
    }

    if let Some(ref value) = self.source_url {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("source_url", json_str);
        }
    }

    if let Some(ref value) = self.webhook {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("webhook", json_str);
        }
    }

    if let Some(ref value) = self.webhook_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("webhook_id", json_str);
        }
    }

    if let Some(ref value) = self.temperature {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("temperature", json_str);
        }
    }

    if let Some(ref value) = self.seed {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("seed", json_str);
        }
    }

    if let Some(ref value) = self.use_multi_channel {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("use_multi_channel", json_str);
        }
    }

    if let Some(ref value) = self.multichannel_output_style {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("multichannel_output_style", json_str);
        }
    }

    if let Some(ref value) = self.webhook_metadata {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("webhook_metadata", json_str);
        }
    }

    if let Some(ref value) = self.entity_detection {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("entity_detection", json_str);
        }
    }

    if let Some(ref value) = self.no_verbatim {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("no_verbatim", json_str);
        }
    }

    if let Some(ref value) = self.use_speaker_library {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("use_speaker_library", json_str);
        }
    }

    if let Some(ref value) = self.detect_speaker_roles {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("detect_speaker_roles", json_str);
        }
    }

    if let Some(ref value) = self.entity_redaction {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("entity_redaction", json_str);
        }
    }

    if let Some(ref value) = self.entity_redaction_mode {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("entity_redaction_mode", json_str);
        }
    }

    if let Some(ref value) = self.keyterms {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("keyterms", json_str);
        }
    }

    form
}
}

impl ConvertRequest3 {
    pub fn builder() -> ConvertRequest3Builder {
        <ConvertRequest3Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertRequest3Builder {
    model_id: Option<SpeechToTextConvertRequestModelId>,
    file: Option<Vec<u8>>,
    language_code: Option<String>,
    tag_audio_events: Option<bool>,
    num_speakers: Option<i64>,
    timestamps_granularity: Option<SpeechToTextConvertRequestTimestampsGranularity>,
    diarize: Option<bool>,
    diarization_threshold: Option<f64>,
    additional_formats: Option<AdditionalFormats>,
    file_format: Option<SpeechToTextConvertRequestFileFormat>,
    cloud_storage_url: Option<String>,
    source_url: Option<String>,
    webhook: Option<bool>,
    webhook_id: Option<String>,
    temperature: Option<f64>,
    seed: Option<i64>,
    use_multi_channel: Option<bool>,
    multichannel_output_style: Option<SpeechToTextConvertRequestMultichannelOutputStyle>,
    webhook_metadata: Option<SpeechToTextConvertRequestWebhookMetadata>,
    entity_detection: Option<SpeechToTextConvertRequestEntityDetection>,
    no_verbatim: Option<bool>,
    use_speaker_library: Option<bool>,
    detect_speaker_roles: Option<bool>,
    entity_redaction: Option<SpeechToTextConvertRequestEntityRedaction>,
    entity_redaction_mode: Option<String>,
    keyterms: Option<Vec<String>>,
    token: Option<String>,
    enable_logging: Option<bool>,
}

impl ConvertRequest3Builder {
    pub fn model_id(mut self, value: SpeechToTextConvertRequestModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn tag_audio_events(mut self, value: bool) -> Self {
        self.tag_audio_events = Some(value);
        self
    }

    pub fn num_speakers(mut self, value: i64) -> Self {
        self.num_speakers = Some(value);
        self
    }

    pub fn timestamps_granularity(mut self, value: SpeechToTextConvertRequestTimestampsGranularity) -> Self {
        self.timestamps_granularity = Some(value);
        self
    }

    pub fn diarize(mut self, value: bool) -> Self {
        self.diarize = Some(value);
        self
    }

    pub fn diarization_threshold(mut self, value: f64) -> Self {
        self.diarization_threshold = Some(value);
        self
    }

    pub fn additional_formats(mut self, value: AdditionalFormats) -> Self {
        self.additional_formats = Some(value);
        self
    }

    pub fn file_format(mut self, value: SpeechToTextConvertRequestFileFormat) -> Self {
        self.file_format = Some(value);
        self
    }

    pub fn cloud_storage_url(mut self, value: impl Into<String>) -> Self {
        self.cloud_storage_url = Some(value.into());
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    pub fn webhook(mut self, value: bool) -> Self {
        self.webhook = Some(value);
        self
    }

    pub fn webhook_id(mut self, value: impl Into<String>) -> Self {
        self.webhook_id = Some(value.into());
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn use_multi_channel(mut self, value: bool) -> Self {
        self.use_multi_channel = Some(value);
        self
    }

    pub fn multichannel_output_style(mut self, value: SpeechToTextConvertRequestMultichannelOutputStyle) -> Self {
        self.multichannel_output_style = Some(value);
        self
    }

    pub fn webhook_metadata(mut self, value: SpeechToTextConvertRequestWebhookMetadata) -> Self {
        self.webhook_metadata = Some(value);
        self
    }

    pub fn entity_detection(mut self, value: SpeechToTextConvertRequestEntityDetection) -> Self {
        self.entity_detection = Some(value);
        self
    }

    pub fn no_verbatim(mut self, value: bool) -> Self {
        self.no_verbatim = Some(value);
        self
    }

    pub fn use_speaker_library(mut self, value: bool) -> Self {
        self.use_speaker_library = Some(value);
        self
    }

    pub fn detect_speaker_roles(mut self, value: bool) -> Self {
        self.detect_speaker_roles = Some(value);
        self
    }

    pub fn entity_redaction(mut self, value: SpeechToTextConvertRequestEntityRedaction) -> Self {
        self.entity_redaction = Some(value);
        self
    }

    pub fn entity_redaction_mode(mut self, value: impl Into<String>) -> Self {
        self.entity_redaction_mode = Some(value.into());
        self
    }

    pub fn keyterms(mut self, value: Vec<String>) -> Self {
        self.keyterms = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConvertRequest3`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model_id`](ConvertRequest3Builder::model_id)
    pub fn build(self) -> Result<ConvertRequest3, BuildError> {
        Ok(ConvertRequest3 {
            model_id: self.model_id.ok_or_else(|| BuildError::missing_field("model_id"))?,
            file: self.file,
            language_code: self.language_code,
            tag_audio_events: self.tag_audio_events,
            num_speakers: self.num_speakers,
            timestamps_granularity: self.timestamps_granularity,
            diarize: self.diarize,
            diarization_threshold: self.diarization_threshold,
            additional_formats: self.additional_formats,
            file_format: self.file_format,
            cloud_storage_url: self.cloud_storage_url,
            source_url: self.source_url,
            webhook: self.webhook,
            webhook_id: self.webhook_id,
            temperature: self.temperature,
            seed: self.seed,
            use_multi_channel: self.use_multi_channel,
            multichannel_output_style: self.multichannel_output_style,
            webhook_metadata: self.webhook_metadata,
            entity_detection: self.entity_detection,
            no_verbatim: self.no_verbatim,
            use_speaker_library: self.use_speaker_library,
            detect_speaker_roles: self.detect_speaker_roles,
            entity_redaction: self.entity_redaction,
            entity_redaction_mode: self.entity_redaction_mode,
            keyterms: self.keyterms,
            token: self.token,
            enable_logging: self.enable_logging,
        })
    }
}
