pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequest8 {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_title_voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_paragraph_voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub from_document: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_content_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_preset: Option<QualityPresetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<ProjectsCreateRequestTargetAudience>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_publication_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mature_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acx_volume_normalization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_normalization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiction: Option<ProjectsCreateRequestFiction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<ProjectsCreateRequestApplyTextNormalization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_assign_voices: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<ProjectsCreateRequestSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_publishing_read: Option<bool>,
}
impl CreateRequest8 {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
    let mut form = reqwest::multipart::Form::new();

    if let Some(ref file_data) = self.from_document {
        form = form.part(
            "from_document",
            reqwest::multipart::Part::bytes(file_data.clone())
                .file_name("from_document")
                .mime_str("application/octet-stream").unwrap()
        );
    }

    if let Ok(json_str) = serde_json::to_string(&self.name) {
        form = form.text("name", json_str);
    }

    if let Some(ref value) = self.default_title_voice_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("default_title_voice_id", json_str);
        }
    }

    if let Some(ref value) = self.default_paragraph_voice_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("default_paragraph_voice_id", json_str);
        }
    }

    if let Some(ref value) = self.default_model_id {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("default_model_id", json_str);
        }
    }

    if let Some(ref value) = self.from_url {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("from_url", json_str);
        }
    }

    if let Some(ref value) = self.from_content_json {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("from_content_json", json_str);
        }
    }

    if let Some(ref value) = self.quality_preset {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("quality_preset", json_str);
        }
    }

    if let Some(ref value) = self.title {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("title", json_str);
        }
    }

    if let Some(ref value) = self.author {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("author", json_str);
        }
    }

    if let Some(ref value) = self.description {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("description", json_str);
        }
    }

    if let Some(ref value) = self.genres {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("genres", json_str);
        }
    }

    if let Some(ref value) = self.target_audience {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("target_audience", json_str);
        }
    }

    if let Some(ref value) = self.language {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("language", json_str);
        }
    }

    if let Some(ref value) = self.content_type {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("content_type", json_str);
        }
    }

    if let Some(ref value) = self.original_publication_date {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("original_publication_date", json_str);
        }
    }

    if let Some(ref value) = self.mature_content {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("mature_content", json_str);
        }
    }

    if let Some(ref value) = self.isbn_number {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("isbn_number", json_str);
        }
    }

    if let Some(ref value) = self.acx_volume_normalization {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("acx_volume_normalization", json_str);
        }
    }

    if let Some(ref value) = self.volume_normalization {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("volume_normalization", json_str);
        }
    }

    if let Some(ref value) = self.pronunciation_dictionary_locators {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("pronunciation_dictionary_locators", json_str);
        }
    }

    if let Some(ref value) = self.callback_url {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("callback_url", json_str);
        }
    }

    if let Some(ref value) = self.fiction {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("fiction", json_str);
        }
    }

    if let Some(ref value) = self.apply_text_normalization {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("apply_text_normalization", json_str);
        }
    }

    if let Some(ref value) = self.auto_convert {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("auto_convert", json_str);
        }
    }

    if let Some(ref value) = self.auto_assign_voices {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("auto_assign_voices", json_str);
        }
    }

    if let Some(ref value) = self.source_type {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("source_type", json_str);
        }
    }

    if let Some(ref value) = self.voice_settings {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("voice_settings", json_str);
        }
    }

    if let Some(ref value) = self.create_publishing_read {
        if let Ok(json_str) = serde_json::to_string(value) {
            form = form.text("create_publishing_read", json_str);
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
    default_title_voice_id: Option<String>,
    default_paragraph_voice_id: Option<String>,
    default_model_id: Option<String>,
    from_url: Option<String>,
    from_document: Option<Vec<u8>>,
    from_content_json: Option<String>,
    quality_preset: Option<QualityPresetType>,
    title: Option<String>,
    author: Option<String>,
    description: Option<String>,
    genres: Option<Vec<String>>,
    target_audience: Option<ProjectsCreateRequestTargetAudience>,
    language: Option<String>,
    content_type: Option<String>,
    original_publication_date: Option<String>,
    mature_content: Option<bool>,
    isbn_number: Option<String>,
    acx_volume_normalization: Option<bool>,
    volume_normalization: Option<bool>,
    pronunciation_dictionary_locators: Option<Vec<String>>,
    callback_url: Option<String>,
    fiction: Option<ProjectsCreateRequestFiction>,
    apply_text_normalization: Option<ProjectsCreateRequestApplyTextNormalization>,
    auto_convert: Option<bool>,
    auto_assign_voices: Option<bool>,
    source_type: Option<ProjectsCreateRequestSourceType>,
    voice_settings: Option<Vec<String>>,
    create_publishing_read: Option<bool>,
}

impl CreateRequest8Builder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn default_title_voice_id(mut self, value: impl Into<String>) -> Self {
        self.default_title_voice_id = Some(value.into());
        self
    }

    pub fn default_paragraph_voice_id(mut self, value: impl Into<String>) -> Self {
        self.default_paragraph_voice_id = Some(value.into());
        self
    }

    pub fn default_model_id(mut self, value: impl Into<String>) -> Self {
        self.default_model_id = Some(value.into());
        self
    }

    pub fn from_url(mut self, value: impl Into<String>) -> Self {
        self.from_url = Some(value.into());
        self
    }

    pub fn from_document(mut self, value: Vec<u8>) -> Self {
        self.from_document = Some(value);
        self
    }

    pub fn from_content_json(mut self, value: impl Into<String>) -> Self {
        self.from_content_json = Some(value.into());
        self
    }

    pub fn quality_preset(mut self, value: QualityPresetType) -> Self {
        self.quality_preset = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn author(mut self, value: impl Into<String>) -> Self {
        self.author = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn genres(mut self, value: Vec<String>) -> Self {
        self.genres = Some(value);
        self
    }

    pub fn target_audience(mut self, value: ProjectsCreateRequestTargetAudience) -> Self {
        self.target_audience = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn original_publication_date(mut self, value: impl Into<String>) -> Self {
        self.original_publication_date = Some(value.into());
        self
    }

    pub fn mature_content(mut self, value: bool) -> Self {
        self.mature_content = Some(value);
        self
    }

    pub fn isbn_number(mut self, value: impl Into<String>) -> Self {
        self.isbn_number = Some(value.into());
        self
    }

    pub fn acx_volume_normalization(mut self, value: bool) -> Self {
        self.acx_volume_normalization = Some(value);
        self
    }

    pub fn volume_normalization(mut self, value: bool) -> Self {
        self.volume_normalization = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<String>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn callback_url(mut self, value: impl Into<String>) -> Self {
        self.callback_url = Some(value.into());
        self
    }

    pub fn fiction(mut self, value: ProjectsCreateRequestFiction) -> Self {
        self.fiction = Some(value);
        self
    }

    pub fn apply_text_normalization(mut self, value: ProjectsCreateRequestApplyTextNormalization) -> Self {
        self.apply_text_normalization = Some(value);
        self
    }

    pub fn auto_convert(mut self, value: bool) -> Self {
        self.auto_convert = Some(value);
        self
    }

    pub fn auto_assign_voices(mut self, value: bool) -> Self {
        self.auto_assign_voices = Some(value);
        self
    }

    pub fn source_type(mut self, value: ProjectsCreateRequestSourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    pub fn voice_settings(mut self, value: Vec<String>) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn create_publishing_read(mut self, value: bool) -> Self {
        self.create_publishing_read = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest8`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateRequest8Builder::name)
    pub fn build(self) -> Result<CreateRequest8, BuildError> {
        Ok(CreateRequest8 {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            default_title_voice_id: self.default_title_voice_id,
            default_paragraph_voice_id: self.default_paragraph_voice_id,
            default_model_id: self.default_model_id,
            from_url: self.from_url,
            from_document: self.from_document,
            from_content_json: self.from_content_json,
            quality_preset: self.quality_preset,
            title: self.title,
            author: self.author,
            description: self.description,
            genres: self.genres,
            target_audience: self.target_audience,
            language: self.language,
            content_type: self.content_type,
            original_publication_date: self.original_publication_date,
            mature_content: self.mature_content,
            isbn_number: self.isbn_number,
            acx_volume_normalization: self.acx_volume_normalization,
            volume_normalization: self.volume_normalization,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            callback_url: self.callback_url,
            fiction: self.fiction,
            apply_text_normalization: self.apply_text_normalization,
            auto_convert: self.auto_convert,
            auto_assign_voices: self.auto_assign_voices,
            source_type: self.source_type,
            voice_settings: self.voice_settings,
            create_publishing_read: self.create_publishing_read,
        })
    }
}
