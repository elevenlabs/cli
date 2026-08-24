pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequest11 {
    #[serde(default)]
    pub files: Vec<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
}
impl CreateRequest11 {
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

impl CreateRequest11 {
    pub fn builder() -> CreateRequest11Builder {
        <CreateRequest11Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequest11Builder {
    files: Option<Vec<Vec<u8>>>,
    remove_background_noise: Option<bool>,
}

impl CreateRequest11Builder {
    pub fn files(mut self, value: Vec<Vec<u8>>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest11`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](CreateRequest11Builder::files)
    pub fn build(self) -> Result<CreateRequest11, BuildError> {
        Ok(CreateRequest11 {
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
            remove_background_noise: self.remove_background_noise,
        })
    }
}
