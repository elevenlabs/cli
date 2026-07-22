pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ManualVerificationFileResponse {
    /// The ID of the file.
    #[serde(default)]
    pub file_id: String,
    /// The name of the file.
    #[serde(default)]
    pub file_name: String,
    /// The MIME type of the file.
    #[serde(default)]
    pub mime_type: String,
    /// The size of the file in bytes.
    #[serde(default)]
    pub size_bytes: i64,
    /// The date of the file in Unix time.
    #[serde(default)]
    pub upload_date_unix: i64,
}

impl ManualVerificationFileResponse {
    pub fn builder() -> ManualVerificationFileResponseBuilder {
        <ManualVerificationFileResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManualVerificationFileResponseBuilder {
    file_id: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    size_bytes: Option<i64>,
    upload_date_unix: Option<i64>,
}

impl ManualVerificationFileResponseBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn upload_date_unix(mut self, value: i64) -> Self {
        self.upload_date_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManualVerificationFileResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](ManualVerificationFileResponseBuilder::file_id)
    /// - [`file_name`](ManualVerificationFileResponseBuilder::file_name)
    /// - [`mime_type`](ManualVerificationFileResponseBuilder::mime_type)
    /// - [`size_bytes`](ManualVerificationFileResponseBuilder::size_bytes)
    /// - [`upload_date_unix`](ManualVerificationFileResponseBuilder::upload_date_unix)
    pub fn build(self) -> Result<ManualVerificationFileResponse, BuildError> {
        Ok(ManualVerificationFileResponse {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
            file_name: self.file_name.ok_or_else(|| BuildError::missing_field("file_name"))?,
            mime_type: self.mime_type.ok_or_else(|| BuildError::missing_field("mime_type"))?,
            size_bytes: self.size_bytes.ok_or_else(|| BuildError::missing_field("size_bytes"))?,
            upload_date_unix: self.upload_date_unix.ok_or_else(|| BuildError::missing_field("upload_date_unix"))?,
        })
    }
}
