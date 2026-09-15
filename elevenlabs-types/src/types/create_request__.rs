pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequest5 {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub hold_audio_file: Vec<u8>,
}
impl CreateRequest5 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "hold_audio_file",
        reqwest::multipart::Part::bytes(self.hold_audio_file.clone())
            .file_name("hold_audio_file")
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
    hold_audio_file: Option<Vec<u8>>,
}

impl CreateRequest5Builder {
    pub fn hold_audio_file(mut self, value: Vec<u8>) -> Self {
        self.hold_audio_file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest5`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hold_audio_file`](CreateRequest5Builder::hold_audio_file)
    pub fn build(self) -> Result<CreateRequest5, BuildError> {
        Ok(CreateRequest5 {
            hold_audio_file: self.hold_audio_file.ok_or_else(|| BuildError::missing_field("hold_audio_file"))?,
        })
    }
}
