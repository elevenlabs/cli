pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateFromFileRequest2 {
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}
impl CreateFromFileRequest2 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    form = form.part(
        "file",
        reqwest::multipart::Part::bytes(self.file.clone())
            .file_name("file")
            .mime_str("application/octet-stream").unwrap()
    );

    if let Some(ref value) = self.name {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("name", json_str);
        }
    }

    if let Some(ref value) = self.parent_folder_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("parent_folder_id", json_str);
        }
    }

    form
}
}

impl CreateFromFileRequest2 {
    pub fn builder() -> CreateFromFileRequest2Builder {
        <CreateFromFileRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFromFileRequest2Builder {
    file: Option<Vec<u8>>,
    name: Option<String>,
    parent_folder_id: Option<String>,
}

impl CreateFromFileRequest2Builder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateFromFileRequest2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](CreateFromFileRequest2Builder::file)
    pub fn build(self) -> Result<CreateFromFileRequest2, BuildError> {
        Ok(CreateFromFileRequest2 {
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
            name: self.name,
            parent_folder_id: self.parent_folder_id,
        })
    }
}
