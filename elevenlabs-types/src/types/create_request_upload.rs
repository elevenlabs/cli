pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequest2 {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessionization: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<AudioNativeCreateRequestApplyTextNormalization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<String>>,
}
impl CreateRequest2 {
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

    if let Ok(json_str) = serde_json::to_string(&self.name) {
        form = form.text("name", json_str);
    }

    if let Some(ref value) = self.image {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("image", json_str);
        }
    }

    if let Some(ref value) = self.author {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("author", json_str);
        }
    }

    if let Some(ref value) = self.title {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("title", json_str);
        }
    }

    if let Some(ref value) = self.small {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("small", json_str);
        }
    }

    if let Some(ref value) = self.text_color {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("text_color", json_str);
        }
    }

    if let Some(ref value) = self.background_color {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("background_color", json_str);
        }
    }

    if let Some(ref value) = self.sessionization {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("sessionization", json_str);
        }
    }

    if let Some(ref value) = self.voice_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("voice_id", json_str);
        }
    }

    if let Some(ref value) = self.model_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("model_id", json_str);
        }
    }

    if let Some(ref value) = self.auto_convert {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("auto_convert", json_str);
        }
    }

    if let Some(ref value) = self.apply_text_normalization {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("apply_text_normalization", json_str);
        }
    }

    if let Some(ref value) = self.pronunciation_dictionary_locators {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("pronunciation_dictionary_locators", json_str);
        }
    }

    form
}
}

impl CreateRequest2 {
    pub fn builder() -> CreateRequest2Builder {
        <CreateRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequest2Builder {
    name: Option<String>,
    image: Option<String>,
    author: Option<String>,
    title: Option<String>,
    small: Option<bool>,
    text_color: Option<String>,
    background_color: Option<String>,
    sessionization: Option<i64>,
    voice_id: Option<String>,
    model_id: Option<String>,
    file: Option<Vec<u8>>,
    auto_convert: Option<bool>,
    apply_text_normalization: Option<AudioNativeCreateRequestApplyTextNormalization>,
    pronunciation_dictionary_locators: Option<Vec<String>>,
}

impl CreateRequest2Builder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn image(mut self, value: impl Into<String>) -> Self {
        self.image = Some(value.into());
        self
    }

    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn small(mut self, value: bool) -> Self {
        self.small = Some(value);
        self
    }

    pub fn text_color(mut self, value: impl Into<String>) -> Self {
        self.text_color = Some(value.into());
        self
    }

    pub fn background_color(mut self, value: impl Into<String>) -> Self {
        self.background_color = Some(value.into());
        self
    }

    pub fn sessionization(mut self, value: i64) -> Self {
        self.sessionization = Some(value);
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn auto_convert(mut self, value: bool) -> Self {
        self.auto_convert = Some(value);
        self
    }

    pub fn apply_text_normalization(mut self, value: AudioNativeCreateRequestApplyTextNormalization) -> Self {
        self.apply_text_normalization = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<String>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateRequest2Builder::name)
    pub fn build(self) -> Result<CreateRequest2, BuildError> {
        Ok(CreateRequest2 {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            image: self.image,
            author: self.author,
            title: self.title,
            small: self.small,
            text_color: self.text_color,
            background_color: self.background_color,
            sessionization: self.sessionization,
            voice_id: self.voice_id,
            model_id: self.model_id,
            file: self.file,
            auto_convert: self.auto_convert,
            apply_text_normalization: self.apply_text_normalization,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
        })
    }
}
