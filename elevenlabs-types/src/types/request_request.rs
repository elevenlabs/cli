pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestRequest {
    #[serde(default)]
    pub files: Vec<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_text: Option<String>,
}
impl RequestRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    for file_data in &self.files {
        form = form.part(
            "files",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("files")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Some(ref value) = self.extra_text {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("extra_text", json_str);
        }
    }

    form
}
}

impl RequestRequest {
    pub fn builder() -> RequestRequestBuilder {
        <RequestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestRequestBuilder {
    files: Option<Vec<Vec<u8>>>,
    extra_text: Option<String>,
}

impl RequestRequestBuilder {
    pub fn files(mut self, value: Vec<Vec<u8>>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn extra_text(mut self, value: impl Into<String>) -> Self {
        self.extra_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RequestRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](RequestRequestBuilder::files)
    pub fn build(self) -> Result<RequestRequest, BuildError> {
        Ok(RequestRequest {
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
            extra_text: self.extra_text,
        })
    }
}
