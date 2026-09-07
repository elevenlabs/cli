pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_composition_plan: Option<MusicUploadRequestExtractCompositionPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_timestamps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_waveform_visual: Option<bool>,
}
impl UploadRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "file",
        reqwest::multipart::Part::bytes(self.file.clone())
            .file_name("file")
            .mime_str("application/octet-stream").unwrap()
    );

    if let Some(ref value) = self.extract_composition_plan {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("extract_composition_plan", json_str);
        }
    }

    if let Some(ref value) = self.with_timestamps {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("with_timestamps", json_str);
        }
    }

    if let Some(ref value) = self.with_waveform_visual {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("with_waveform_visual", json_str);
        }
    }

    form
}
}

impl UploadRequest {
    pub fn builder() -> UploadRequestBuilder {
        <UploadRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadRequestBuilder {
    file: Option<Vec<u8>>,
    extract_composition_plan: Option<MusicUploadRequestExtractCompositionPlan>,
    with_timestamps: Option<bool>,
    with_waveform_visual: Option<bool>,
}

impl UploadRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn extract_composition_plan(mut self, value: MusicUploadRequestExtractCompositionPlan) -> Self {
        self.extract_composition_plan = Some(value);
        self
    }

    pub fn with_timestamps(mut self, value: bool) -> Self {
        self.with_timestamps = Some(value);
        self
    }

    pub fn with_waveform_visual(mut self, value: bool) -> Self {
        self.with_waveform_visual = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UploadRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](UploadRequestBuilder::file)
    pub fn build(self) -> Result<UploadRequest, BuildError> {
        Ok(UploadRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            extract_composition_plan: self.extract_composition_plan,
            with_timestamps: self.with_timestamps,
            with_waveform_visual: self.with_waveform_visual,
        })
    }
}
