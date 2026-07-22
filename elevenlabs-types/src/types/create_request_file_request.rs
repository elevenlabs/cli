pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequest3 {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(default)]
    pub text: String,
}
impl CreateRequest3 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "file",
        reqwest::multipart::Part::bytes(self.file.clone())
            .file_name("file")
            .mime_str("application/octet-stream").unwrap()
    );

    if let Ok(json_str) = serde_json::to_string(&self.text) {
        form = form.text("text", json_str);
    }

    form
}
}

impl CreateRequest3 {
    pub fn builder() -> CreateRequest3Builder {
        <CreateRequest3Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequest3Builder {
    file: Option<Vec<u8>>,
    text: Option<String>,
}

impl CreateRequest3Builder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest3`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](CreateRequest3Builder::file)
    /// - [`text`](CreateRequest3Builder::text)
    pub fn build(self) -> Result<CreateRequest3, BuildError> {
        Ok(CreateRequest3 {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
