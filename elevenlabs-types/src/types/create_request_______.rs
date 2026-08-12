pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequest10 {
    #[serde(default)]
    pub files: Vec<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
}
impl CreateRequest10 {
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

    if let Some(ref value) = self.remove_background_noise {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("remove_background_noise", json_str);
        }
    }

    form
}
}

impl CreateRequest10 {
    pub fn builder() -> CreateRequest10Builder {
        <CreateRequest10Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequest10Builder {
    files: Option<Vec<Vec<u8>>>,
    remove_background_noise: Option<bool>,
}

impl CreateRequest10Builder {
    pub fn files(mut self, value: Vec<Vec<u8>>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest10`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](CreateRequest10Builder::files)
    pub fn build(self) -> Result<CreateRequest10, BuildError> {
        Ok(CreateRequest10 {
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
            remove_background_noise: self.remove_background_noise,
        })
    }
}
