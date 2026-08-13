pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddToKnowledgeBaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(skip)]
    pub agent_id: Option<String>,
}
impl AddToKnowledgeBaseRequest {
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

    if let Some(ref value) = self.name {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("name", json_str);
        }
    }

    if let Some(ref value) = self.url {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("url", json_str);
        }
    }

    form
}
}

impl AddToKnowledgeBaseRequest {
    pub fn builder() -> AddToKnowledgeBaseRequestBuilder {
        <AddToKnowledgeBaseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddToKnowledgeBaseRequestBuilder {
    name: Option<String>,
    url: Option<String>,
    file: Option<Vec<u8>>,
    agent_id: Option<String>,
}

impl AddToKnowledgeBaseRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddToKnowledgeBaseRequest`].
    pub fn build(self) -> Result<AddToKnowledgeBaseRequest, BuildError> {
        Ok(AddToKnowledgeBaseRequest {
            name: self.name,
            url: self.url,
            file: self.file,
            agent_id: self.agent_id,
        })
    }
}
