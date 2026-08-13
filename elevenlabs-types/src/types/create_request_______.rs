pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRequest10 {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub files: Vec<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<AddVoiceRequestLabels>,
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

    if let Ok(json_str) = serde_json::to_string(&self.name) {
        form = form.text("name", json_str);
    }

    if let Some(ref value) = self.remove_background_noise {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("remove_background_noise", json_str);
        }
    }

    if let Some(ref value) = self.description {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("description", json_str);
        }
    }

    if let Some(ref value) = self.labels {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("labels", json_str);
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
    name: Option<String>,
    files: Option<Vec<Vec<u8>>>,
    remove_background_noise: Option<bool>,
    description: Option<String>,
    labels: Option<AddVoiceRequestLabels>,
}

impl CreateRequest10Builder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn files(mut self, value: Vec<Vec<u8>>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn labels(mut self, value: AddVoiceRequestLabels) -> Self {
        self.labels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest10`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateRequest10Builder::name)
    /// - [`files`](CreateRequest10Builder::files)
    pub fn build(self) -> Result<CreateRequest10, BuildError> {
        Ok(CreateRequest10 {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
            remove_background_noise: self.remove_background_noise,
            description: self.description,
            labels: self.labels,
        })
    }
}
