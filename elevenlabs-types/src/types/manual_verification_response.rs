pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ManualVerificationResponse {
    /// The extra text of the manual verification.
    #[serde(default)]
    pub extra_text: String,
    /// The date of the manual verification in Unix time.
    #[serde(default)]
    pub request_time_unix: i64,
    /// The files of the manual verification.
    #[serde(default)]
    pub files: Vec<ManualVerificationFileResponse>,
}

impl ManualVerificationResponse {
    pub fn builder() -> ManualVerificationResponseBuilder {
        <ManualVerificationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManualVerificationResponseBuilder {
    extra_text: Option<String>,
    request_time_unix: Option<i64>,
    files: Option<Vec<ManualVerificationFileResponse>>,
}

impl ManualVerificationResponseBuilder {
    pub fn extra_text(mut self, value: impl Into<String>) -> Self {
        self.extra_text = Some(value.into());
        self
    }

    pub fn request_time_unix(mut self, value: i64) -> Self {
        self.request_time_unix = Some(value);
        self
    }

    pub fn files(mut self, value: Vec<ManualVerificationFileResponse>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManualVerificationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`extra_text`](ManualVerificationResponseBuilder::extra_text)
    /// - [`request_time_unix`](ManualVerificationResponseBuilder::request_time_unix)
    /// - [`files`](ManualVerificationResponseBuilder::files)
    pub fn build(self) -> Result<ManualVerificationResponse, BuildError> {
        Ok(ManualVerificationResponse {
            extra_text: self.extra_text.ok_or_else(|| BuildError::missing_field("extra_text"))?,
            request_time_unix: self.request_time_unix.ok_or_else(|| BuildError::missing_field("request_time_unix"))?,
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
        })
    }
}
