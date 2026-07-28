pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateRequest3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub from_document: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_content_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert: Option<bool>,
}
impl UpdateRequest3 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    if let Some(ref file_data) = self.from_document {
        form = form.part(
            "from_document",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("from_document")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Some(ref value) = self.from_url {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("from_url", json_str);
        }
    }

    if let Some(ref value) = self.from_content_json {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("from_content_json", json_str);
        }
    }

    if let Some(ref value) = self.auto_convert {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("auto_convert", json_str);
        }
    }

    form
}
}

impl UpdateRequest3 {
    pub fn builder() -> UpdateRequest3Builder {
        <UpdateRequest3Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRequest3Builder {
    from_url: Option<String>,
    from_document: Option<Vec<u8>>,
    from_content_json: Option<String>,
    auto_convert: Option<bool>,
}

impl UpdateRequest3Builder {
    pub fn from_url(mut self, value: impl Into<String>) -> Self {
        self.from_url = Some(value.into());
        self
    }

    pub fn from_document(mut self, value: Vec<u8>) -> Self {
        self.from_document = Some(value);
        self
    }

    pub fn from_content_json(mut self, value: impl Into<String>) -> Self {
        self.from_content_json = Some(value.into());
        self
    }

    pub fn auto_convert(mut self, value: bool) -> Self {
        self.auto_convert = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateRequest3`].
    pub fn build(self) -> Result<UpdateRequest3, BuildError> {
        Ok(UpdateRequest3 {
            from_url: self.from_url,
            from_document: self.from_document,
            from_content_json: self.from_content_json,
            auto_convert: self.auto_convert,
        })
    }
}
