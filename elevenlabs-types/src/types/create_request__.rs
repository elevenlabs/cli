pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequest5 {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
}
impl CreateRequest5 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "file",
        reqwest::multipart::Part::bytes(self.file.clone())
            .file_name("file")
            .mime_str("application/octet-stream").unwrap()
    );

    form
}
}

impl CreateRequest5 {
    pub fn builder() -> CreateRequest5Builder {
        <CreateRequest5Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequest5Builder {
    file: Option<Vec<u8>>,
}

impl CreateRequest5Builder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest5`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](CreateRequest5Builder::file)
    pub fn build(self) -> Result<CreateRequest5, BuildError> {
        Ok(CreateRequest5 {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}
