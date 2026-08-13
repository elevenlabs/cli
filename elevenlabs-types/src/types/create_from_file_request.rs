pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateFromFileRequest {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_access: Option<AddPronunciationDictionaryRequestWorkspaceAccess>,
}
impl CreateFromFileRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    if let Some(ref file_data) = self.file {
        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("file")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Ok(json_str) = serde_json::to_string(&self.name) {
        form = form.text("name", json_str);
    }

    if let Some(ref value) = self.description {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("description", json_str);
        }
    }

    if let Some(ref value) = self.workspace_access {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("workspace_access", json_str);
        }
    }

    form
}
}

impl CreateFromFileRequest {
    pub fn builder() -> CreateFromFileRequestBuilder {
        <CreateFromFileRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFromFileRequestBuilder {
    name: Option<String>,
    file: Option<Vec<u8>>,
    description: Option<String>,
    workspace_access: Option<AddPronunciationDictionaryRequestWorkspaceAccess>,
}

impl CreateFromFileRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn workspace_access(mut self, value: AddPronunciationDictionaryRequestWorkspaceAccess) -> Self {
        self.workspace_access = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateFromFileRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateFromFileRequestBuilder::name)
    pub fn build(self) -> Result<CreateFromFileRequest, BuildError> {
        Ok(CreateFromFileRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            file: self.file,
            description: self.description,
            workspace_access: self.workspace_access,
        })
    }
}
