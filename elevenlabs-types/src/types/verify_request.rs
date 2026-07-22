pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VerifyRequest {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub recording: Vec<u8>,
}
impl VerifyRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "recording",
        reqwest::multipart::Part::bytes(self.recording.clone())
            .file_name("recording")
            .mime_str("application/octet-stream").unwrap()
    );

    form
}
}

impl VerifyRequest {
    pub fn builder() -> VerifyRequestBuilder {
        <VerifyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerifyRequestBuilder {
    recording: Option<Vec<u8>>,
}

impl VerifyRequestBuilder {
    pub fn recording(mut self, value: Vec<u8>) -> Self {
        self.recording = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VerifyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recording`](VerifyRequestBuilder::recording)
    pub fn build(self) -> Result<VerifyRequest, BuildError> {
        Ok(VerifyRequest {
            recording: self.recording.ok_or_else(|| BuildError::missing_field("recording"))?,
        })
    }
}
