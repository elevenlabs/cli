pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRequest6 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<ProjectCreateRequestModelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyterms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub transcript: Option<Vec<u8>>,
}
impl CreateRequest6 {
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

    if let Some(ref file_data) = self.transcript {
        form = form.part(
            "transcript",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("transcript")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Some(ref value) = self.source_url {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("source_url", json_str);
        }
    }

    if let Some(ref value) = self.reference {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("reference", json_str);
        }
    }

    if let Some(ref value) = self.source_language {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("source_language", json_str);
        }
    }

    if let Some(ref value) = self.model_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("model_id", json_str);
        }
    }

    if let Some(ref value) = self.keyterms {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("keyterms", json_str);
        }
    }

    if let Some(ref value) = self.webhook_ids {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("webhook_ids", json_str);
        }
    }

    if let Some(ref value) = self.target_language {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("target_language", json_str);
        }
    }

    form
}
}

impl CreateRequest6 {
    pub fn builder() -> CreateRequest6Builder {
        <CreateRequest6Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequest6Builder {
    file: Option<Vec<u8>>,
    source_url: Option<String>,
    reference: Option<String>,
    source_language: Option<String>,
    model_id: Option<ProjectCreateRequestModelId>,
    keyterms: Option<Vec<String>>,
    webhook_ids: Option<Vec<String>>,
    target_language: Option<String>,
    transcript: Option<Vec<u8>>,
}

impl CreateRequest6Builder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn source_language(mut self, value: impl Into<String>) -> Self {
        self.source_language = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: ProjectCreateRequestModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn keyterms(mut self, value: Vec<String>) -> Self {
        self.keyterms = Some(value);
        self
    }

    pub fn webhook_ids(mut self, value: Vec<String>) -> Self {
        self.webhook_ids = Some(value);
        self
    }

    pub fn target_language(mut self, value: impl Into<String>) -> Self {
        self.target_language = Some(value.into());
        self
    }

    pub fn transcript(mut self, value: Vec<u8>) -> Self {
        self.transcript = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest6`].
    pub fn build(self) -> Result<CreateRequest6, BuildError> {
        Ok(CreateRequest6 {
            file: self.file,
            source_url: self.source_url,
            reference: self.reference,
            source_language: self.source_language,
            model_id: self.model_id,
            keyterms: self.keyterms,
            webhook_ids: self.webhook_ids,
            target_language: self.target_language,
            transcript: self.transcript,
        })
    }
}
