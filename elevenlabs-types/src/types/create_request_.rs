pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequest4 {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub avatar_file: Vec<u8>,
}
impl CreateRequest4 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "avatar_file",
        reqwest::multipart::Part::bytes(self.avatar_file.clone())
            .file_name("avatar_file")
            .mime_str("application/octet-stream").unwrap()
    );

    form
}
}

impl CreateRequest4 {
    pub fn builder() -> CreateRequest4Builder {
        <CreateRequest4Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequest4Builder {
    avatar_file: Option<Vec<u8>>,
}

impl CreateRequest4Builder {
    pub fn avatar_file(mut self, value: Vec<u8>) -> Self {
        self.avatar_file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest4`].
    /// This method will fail if any of the following fields are not set:
    /// - [`avatar_file`](CreateRequest4Builder::avatar_file)
    pub fn build(self) -> Result<CreateRequest4, BuildError> {
        Ok(CreateRequest4 {
            avatar_file: self.avatar_file.ok_or_else(|| BuildError::missing_field("avatar_file"))?,
        })
    }
}
