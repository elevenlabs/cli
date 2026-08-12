pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConvertRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub audio: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<AudioIsolationRequestFileFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_b64: Option<String>,
}
impl ConvertRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "audio",
        reqwest::multipart::Part::bytes(self.audio.clone())
            .file_name("audio")
            .mime_str("application/octet-stream").unwrap()
    );

    if let Some(ref value) = self.file_format {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("file_format", json_str);
        }
    }

    if let Some(ref value) = self.preview_b64 {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("preview_b64", json_str);
        }
    }

    form
}
}

impl ConvertRequest {
    pub fn builder() -> ConvertRequestBuilder {
        <ConvertRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertRequestBuilder {
    audio: Option<Vec<u8>>,
    file_format: Option<AudioIsolationRequestFileFormat>,
    preview_b64: Option<String>,
}

impl ConvertRequestBuilder {
    pub fn audio(mut self, value: Vec<u8>) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn file_format(mut self, value: AudioIsolationRequestFileFormat) -> Self {
        self.file_format = Some(value);
        self
    }

    pub fn preview_b64(mut self, value: impl Into<String>) -> Self {
        self.preview_b64 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvertRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](ConvertRequestBuilder::audio)
    pub fn build(self) -> Result<ConvertRequest, BuildError> {
        Ok(ConvertRequest {
            audio: self.audio.ok_or_else(|| BuildError::missing_field("audio"))?,
            file_format: self.file_format,
            preview_b64: self.preview_b64,
        })
    }
}
