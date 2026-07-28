pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConvertRequest2 {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub audio: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<SpeechToSpeechRequestFileFormat>,
    #[serde(skip)]
    pub enable_logging: Option<bool>,
    #[serde(skip)]
    pub optimize_streaming_latency: Option<i64>,
    #[serde(skip)]
    pub output_format: Option<SpeechToSpeechConvertRequestOutputFormat>,
}
impl ConvertRequest2 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "audio",
        reqwest::multipart::Part::bytes(self.audio.clone())
            .file_name("audio")
            .mime_str("application/octet-stream").unwrap()
    );

    if let Some(ref value) = self.model_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("model_id", json_str);
        }
    }

    if let Some(ref value) = self.voice_settings {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("voice_settings", json_str);
        }
    }

    if let Some(ref value) = self.seed {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("seed", json_str);
        }
    }

    if let Some(ref value) = self.remove_background_noise {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("remove_background_noise", json_str);
        }
    }

    if let Some(ref value) = self.file_format {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("file_format", json_str);
        }
    }

    form
}
}

impl ConvertRequest2 {
    pub fn builder() -> ConvertRequest2Builder {
        <ConvertRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertRequest2Builder {
    audio: Option<Vec<u8>>,
    model_id: Option<String>,
    voice_settings: Option<String>,
    seed: Option<i64>,
    remove_background_noise: Option<bool>,
    file_format: Option<SpeechToSpeechRequestFileFormat>,
    enable_logging: Option<bool>,
    optimize_streaming_latency: Option<i64>,
    output_format: Option<SpeechToSpeechConvertRequestOutputFormat>,
}

impl ConvertRequest2Builder {
    pub fn audio(mut self, value: Vec<u8>) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn voice_settings(mut self, value: impl Into<String>) -> Self {
        self.voice_settings = Some(value.into());
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    pub fn file_format(mut self, value: SpeechToSpeechRequestFileFormat) -> Self {
        self.file_format = Some(value);
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

    pub fn output_format(mut self, value: SpeechToSpeechConvertRequestOutputFormat) -> Self {
        self.output_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConvertRequest2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](ConvertRequest2Builder::audio)
    pub fn build(self) -> Result<ConvertRequest2, BuildError> {
        Ok(ConvertRequest2 {
            audio: self.audio.ok_or_else(|| BuildError::missing_field("audio"))?,
            model_id: self.model_id,
            voice_settings: self.voice_settings,
            seed: self.seed,
            remove_background_noise: self.remove_background_noise,
            file_format: self.file_format,
            enable_logging: self.enable_logging,
            optimize_streaming_latency: self.optimize_streaming_latency,
            output_format: self.output_format,
        })
    }
}
