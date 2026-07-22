pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub csv_file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub foreground_audio_file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub background_audio_file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_accent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_speakers: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_resolution: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_background_audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_profanity_filter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dubbing_studio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_voice_cloning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<DubRequestMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub csv_fps: Option<f64>,
}
impl CreateRequest {
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

    if let Some(ref file_data) = self.csv_file {
        form = form.part(
            "csv_file",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("csv_file")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Some(ref file_data) = self.foreground_audio_file {
        form = form.part(
            "foreground_audio_file",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("foreground_audio_file")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Some(ref file_data) = self.background_audio_file {
        form = form.part(
            "background_audio_file",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("background_audio_file")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Some(ref value) = self.name {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("name", json_str);
        }
    }

    if let Some(ref value) = self.source_url {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("source_url", json_str);
        }
    }

    if let Some(ref value) = self.source_lang {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("source_lang", json_str);
        }
    }

    if let Some(ref value) = self.target_lang {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("target_lang", json_str);
        }
    }

    if let Some(ref value) = self.target_accent {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("target_accent", json_str);
        }
    }

    if let Some(ref value) = self.num_speakers {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("num_speakers", json_str);
        }
    }

    if let Some(ref value) = self.watermark {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("watermark", json_str);
        }
    }

    if let Some(ref value) = self.start_time {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("start_time", json_str);
        }
    }

    if let Some(ref value) = self.end_time {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("end_time", json_str);
        }
    }

    if let Some(ref value) = self.highest_resolution {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("highest_resolution", json_str);
        }
    }

    if let Some(ref value) = self.drop_background_audio {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("drop_background_audio", json_str);
        }
    }

    if let Some(ref value) = self.use_profanity_filter {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("use_profanity_filter", json_str);
        }
    }

    if let Some(ref value) = self.dubbing_studio {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("dubbing_studio", json_str);
        }
    }

    if let Some(ref value) = self.disable_voice_cloning {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("disable_voice_cloning", json_str);
        }
    }

    if let Some(ref value) = self.mode {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("mode", json_str);
        }
    }

    if let Some(ref value) = self.csv_fps {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("csv_fps", json_str);
        }
    }

    form
}
}

impl CreateRequest {
    pub fn builder() -> CreateRequestBuilder {
        <CreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestBuilder {
    file: Option<Vec<u8>>,
    csv_file: Option<Vec<u8>>,
    foreground_audio_file: Option<Vec<u8>>,
    background_audio_file: Option<Vec<u8>>,
    name: Option<String>,
    source_url: Option<String>,
    source_lang: Option<String>,
    target_lang: Option<String>,
    target_accent: Option<String>,
    num_speakers: Option<i64>,
    watermark: Option<bool>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    highest_resolution: Option<bool>,
    drop_background_audio: Option<bool>,
    use_profanity_filter: Option<bool>,
    dubbing_studio: Option<bool>,
    disable_voice_cloning: Option<bool>,
    mode: Option<DubRequestMode>,
    csv_fps: Option<f64>,
}

impl CreateRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn csv_file(mut self, value: Vec<u8>) -> Self {
        self.csv_file = Some(value);
        self
    }

    pub fn foreground_audio_file(mut self, value: Vec<u8>) -> Self {
        self.foreground_audio_file = Some(value);
        self
    }

    pub fn background_audio_file(mut self, value: Vec<u8>) -> Self {
        self.background_audio_file = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    pub fn source_lang(mut self, value: impl Into<String>) -> Self {
        self.source_lang = Some(value.into());
        self
    }

    pub fn target_lang(mut self, value: impl Into<String>) -> Self {
        self.target_lang = Some(value.into());
        self
    }

    pub fn target_accent(mut self, value: impl Into<String>) -> Self {
        self.target_accent = Some(value.into());
        self
    }

    pub fn num_speakers(mut self, value: i64) -> Self {
        self.num_speakers = Some(value);
        self
    }

    pub fn watermark(mut self, value: bool) -> Self {
        self.watermark = Some(value);
        self
    }

    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn highest_resolution(mut self, value: bool) -> Self {
        self.highest_resolution = Some(value);
        self
    }

    pub fn drop_background_audio(mut self, value: bool) -> Self {
        self.drop_background_audio = Some(value);
        self
    }

    pub fn use_profanity_filter(mut self, value: bool) -> Self {
        self.use_profanity_filter = Some(value);
        self
    }

    pub fn dubbing_studio(mut self, value: bool) -> Self {
        self.dubbing_studio = Some(value);
        self
    }

    pub fn disable_voice_cloning(mut self, value: bool) -> Self {
        self.disable_voice_cloning = Some(value);
        self
    }

    pub fn mode(mut self, value: DubRequestMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn csv_fps(mut self, value: f64) -> Self {
        self.csv_fps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest`].
    pub fn build(self) -> Result<CreateRequest, BuildError> {
        Ok(CreateRequest {
            file: self.file,
            csv_file: self.csv_file,
            foreground_audio_file: self.foreground_audio_file,
            background_audio_file: self.background_audio_file,
            name: self.name,
            source_url: self.source_url,
            source_lang: self.source_lang,
            target_lang: self.target_lang,
            target_accent: self.target_accent,
            num_speakers: self.num_speakers,
            watermark: self.watermark,
            start_time: self.start_time,
            end_time: self.end_time,
            highest_resolution: self.highest_resolution,
            drop_background_audio: self.drop_background_audio,
            use_profanity_filter: self.use_profanity_filter,
            dubbing_studio: self.dubbing_studio,
            disable_voice_cloning: self.disable_voice_cloning,
            mode: self.mode,
            csv_fps: self.csv_fps,
        })
    }
}
