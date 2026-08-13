pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequest8 {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub primary_genre: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Vec<u8>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<FinetunesCreateRequestVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<FinetunesCreateRequestModelId>,
}
impl CreateRequest8 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    if let Some(ref files) = self.files {
        for file_data in files {
            form = form.part(
                "files",
                reqwest::multipart::Part::bytes(file_data.clone())
                    .file_name("files")
                    .mime_str("application/octet-stream").unwrap()
            );
        }
    }

    if let Ok(json_str) = serde_json::to_string(&self.name) {
        form = form.text("name", json_str);
    }

    if let Ok(json_str) = serde_json::to_string(&self.primary_genre) {
        form = form.text("primary_genre", json_str);
    }

    if let Some(ref value) = self.tags {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("tags", json_str);
        }
    }

    if let Some(ref value) = self.visibility {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("visibility", json_str);
        }
    }

    if let Some(ref value) = self.model_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("model_id", json_str);
        }
    }

    form
}
}

impl CreateRequest8 {
    pub fn builder() -> CreateRequest8Builder {
        <CreateRequest8Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequest8Builder {
    name: Option<String>,
    primary_genre: Option<String>,
    files: Option<Vec<Vec<u8>>>,
    tags: Option<Vec<String>>,
    visibility: Option<FinetunesCreateRequestVisibility>,
    model_id: Option<FinetunesCreateRequestModelId>,
}

impl CreateRequest8Builder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn primary_genre(mut self, value: impl Into<String>) -> Self {
        self.primary_genre = Some(value.into());
        self
    }

    pub fn files(mut self, value: Vec<Vec<u8>>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn visibility(mut self, value: FinetunesCreateRequestVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn model_id(mut self, value: FinetunesCreateRequestModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest8`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateRequest8Builder::name)
    /// - [`primary_genre`](CreateRequest8Builder::primary_genre)
    pub fn build(self) -> Result<CreateRequest8, BuildError> {
        Ok(CreateRequest8 {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            primary_genre: self.primary_genre.ok_or_else(|| BuildError::missing_field("primary_genre"))?,
            files: self.files,
            tags: self.tags,
            visibility: self.visibility,
            model_id: self.model_id,
        })
    }
}
