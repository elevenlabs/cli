pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateRequest2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_publish: Option<bool>,
}
impl UpdateRequest2 {
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

    if let Some(ref value) = self.auto_convert {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("auto_convert", json_str);
        }
    }

    if let Some(ref value) = self.auto_publish {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("auto_publish", json_str);
        }
    }

    form
}
}

impl UpdateRequest2 {
    pub fn builder() -> UpdateRequest2Builder {
        <UpdateRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRequest2Builder {
    file: Option<Vec<u8>>,
    auto_convert: Option<bool>,
    auto_publish: Option<bool>,
}

impl UpdateRequest2Builder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn auto_convert(mut self, value: bool) -> Self {
        self.auto_convert = Some(value);
        self
    }

    pub fn auto_publish(mut self, value: bool) -> Self {
        self.auto_publish = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateRequest2`].
    pub fn build(self) -> Result<UpdateRequest2, BuildError> {
        Ok(UpdateRequest2 {
            file: self.file,
            auto_convert: self.auto_convert,
            auto_publish: self.auto_publish,
        })
    }
}
