pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WatermarkRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl WatermarkRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "file",
        reqwest::multipart::Part::bytes(self.file.clone())
            .file_name("file")
            .mime_str("application/octet-stream").unwrap()
    );

    if let Some(ref value) = self.version {
        form = form.text("version", value.clone());
    }

    form
}
}

impl WatermarkRequest {
    pub fn builder() -> WatermarkRequestBuilder {
        <WatermarkRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WatermarkRequestBuilder {
    file: Option<Vec<u8>>,
    version: Option<String>,
}

impl WatermarkRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WatermarkRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](WatermarkRequestBuilder::file)
    pub fn build(self) -> Result<WatermarkRequest, BuildError> {
        Ok(WatermarkRequest {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            version: self.version,
        })
    }
}
