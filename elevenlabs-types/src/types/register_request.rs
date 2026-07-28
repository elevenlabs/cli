pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegisterRequest {
    #[serde(default)]
    pub declared_language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub media: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url_content_type: Option<String>,
}
impl RegisterRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    if let Some(ref file_data) = self.media {
        form = form.part(
            "media",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("media")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Ok(json_str) = serde_json::to_string(&self.declared_language) {
        form = form.text("declared_language", json_str);
    }

    if let Some(ref value) = self.media_url {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("media_url", json_str);
        }
    }

    if let Some(ref value) = self.media_url_filename {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("media_url_filename", json_str);
        }
    }

    if let Some(ref value) = self.media_url_content_type {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("media_url_content_type", json_str);
        }
    }

    form
}
}

impl RegisterRequest {
    pub fn builder() -> RegisterRequestBuilder {
        <RegisterRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterRequestBuilder {
    declared_language: Option<String>,
    media: Option<Vec<u8>>,
    media_url: Option<String>,
    media_url_filename: Option<String>,
    media_url_content_type: Option<String>,
}

impl RegisterRequestBuilder {
    pub fn declared_language(mut self, value: impl Into<String>) -> Self {
        self.declared_language = Some(value.into());
        self
    }

    pub fn media(mut self, value: Vec<u8>) -> Self {
        self.media = Some(value);
        self
    }

    pub fn media_url(mut self, value: impl Into<String>) -> Self {
        self.media_url = Some(value.into());
        self
    }

    pub fn media_url_filename(mut self, value: impl Into<String>) -> Self {
        self.media_url_filename = Some(value.into());
        self
    }

    pub fn media_url_content_type(mut self, value: impl Into<String>) -> Self {
        self.media_url_content_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`declared_language`](RegisterRequestBuilder::declared_language)
    pub fn build(self) -> Result<RegisterRequest, BuildError> {
        Ok(RegisterRequest {
            declared_language: self.declared_language.ok_or_else(|| BuildError::missing_field("declared_language"))?,
            media: self.media,
            media_url: self.media_url,
            media_url_filename: self.media_url_filename,
            media_url_content_type: self.media_url_content_type,
        })
    }
}
