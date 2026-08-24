pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectExtendedResponse {
    /// The ID of the project.
    #[serde(default)]
    pub project_id: String,
    /// The name of the project.
    #[serde(default)]
    pub name: String,
    /// The creation date of the project.
    #[serde(default)]
    pub create_date_unix: i64,
    /// The user ID who created the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    /// The default title project voice reference ID.
    #[serde(default)]
    pub default_title_voice_ref_id: String,
    /// The default paragraph project voice reference ID.
    #[serde(default)]
    pub default_paragraph_voice_ref_id: String,
    /// The default model ID.
    #[serde(default)]
    pub default_model_id: String,
    /// The last conversion date of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_conversion_date_unix: Option<i64>,
    /// Whether the project can be downloaded.
    #[serde(default)]
    pub can_be_downloaded: bool,
    /// The title of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The author of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// The description of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of genres of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    /// The cover image URL of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image_url: Option<String>,
    /// The target audience of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<ProjectExtendedResponseTargetAudience>,
    /// Two-letter language code (ISO 639-1) of the language of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The content type of the project, e.g. 'Novel' or 'Short Story'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The original publication date of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_publication_date: Option<String>,
    /// Whether the project contains mature content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mature_content: Option<bool>,
    /// The ISBN number of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn_number: Option<String>,
    /// Whether the project uses volume normalization.
    #[serde(default)]
    pub volume_normalization: bool,
    /// The state of the project.
    pub state: ProjectState,
    /// The access level of the project.
    pub access_level: ProjectExtendedResponseAccessLevel,
    /// Whether the project is fiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiction: Option<ProjectExtendedResponseFiction>,
    /// Whether quality check is enabled for this project.
    #[serde(default)]
    pub quality_check_on: bool,
    /// Whether quality check is enabled on the project when bulk converting.
    #[serde(default)]
    pub quality_check_on_when_bulk_convert: bool,
    /// The creation meta of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_meta: Option<ProjectCreationMetaResponseModel>,
    /// The source type of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<ProjectExtendedResponseSourceType>,
    /// Whether chapters are enabled for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters_enabled: Option<bool>,
    /// Whether captions are enabled for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captions_enabled: Option<bool>,
    /// Global styling to be applied to all captions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_style: Option<CaptionStyleModel>,
    /// Styling changes that have been made to the provided templates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_style_template_overrides: Option<HashMap<String, Option<CaptionStyleModel>>>,
    /// The public share ID of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_share_id: Option<String>,
    /// The aspect ratio of the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<ProjectExtendedResponseAspectRatio>,
    /// Agent-related settings for the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_settings: Option<StudioAgentSettingsModel>,
    /// The quality preset level of the project.
    pub quality_preset: QualityPresetType,
    /// List of chapters of the project and their metadata.
    #[serde(default)]
    pub chapters: Vec<ChapterResponse>,
    /// List of pronunciation dictionary versions of the project and their metadata.
    #[serde(default)]
    pub pronunciation_dictionary_versions: Vec<PronunciationDictionaryVersionResponseModel>,
    /// List of pronunciation dictionary locators.
    #[serde(default)]
    pub pronunciation_dictionary_locators: Vec<PronunciationDictionaryLocatorResponseModel>,
    /// Whether text normalization is applied to the project.
    pub apply_text_normalization: ProjectExtendedResponseApplyTextNormalization,
    /// Experimental features for the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, serde_json::Value>>,
    /// List of uploaded assets e.g. videos, audios.
    #[serde(default)]
    pub assets: Vec<ProjectExtendedResponseAssetsItem>,
    /// List of configured project voices.
    #[serde(default)]
    pub voices: Vec<ProjectVoiceResponseModel>,
    /// List of voices used by the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_voices: Option<Vec<Voice>>,
    /// The ElevenReader data if the book was published.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishing_read: Option<DirectPublishingReadResponseModel>,
    /// The default title voice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_title_voice_id: Option<String>,
    /// The default paragraph voice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_paragraph_voice_id: Option<String>,
}

impl ProjectExtendedResponse {
    pub fn builder() -> ProjectExtendedResponseBuilder {
        <ProjectExtendedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectExtendedResponseBuilder {
    project_id: Option<String>,
    name: Option<String>,
    create_date_unix: Option<i64>,
    created_by_user_id: Option<String>,
    default_title_voice_ref_id: Option<String>,
    default_paragraph_voice_ref_id: Option<String>,
    default_model_id: Option<String>,
    last_conversion_date_unix: Option<i64>,
    can_be_downloaded: Option<bool>,
    title: Option<String>,
    author: Option<String>,
    description: Option<String>,
    genres: Option<Vec<String>>,
    cover_image_url: Option<String>,
    target_audience: Option<ProjectExtendedResponseTargetAudience>,
    language: Option<String>,
    content_type: Option<String>,
    original_publication_date: Option<String>,
    mature_content: Option<bool>,
    isbn_number: Option<String>,
    volume_normalization: Option<bool>,
    state: Option<ProjectState>,
    access_level: Option<ProjectExtendedResponseAccessLevel>,
    fiction: Option<ProjectExtendedResponseFiction>,
    quality_check_on: Option<bool>,
    quality_check_on_when_bulk_convert: Option<bool>,
    creation_meta: Option<ProjectCreationMetaResponseModel>,
    source_type: Option<ProjectExtendedResponseSourceType>,
    chapters_enabled: Option<bool>,
    captions_enabled: Option<bool>,
    caption_style: Option<CaptionStyleModel>,
    caption_style_template_overrides: Option<HashMap<String, Option<CaptionStyleModel>>>,
    public_share_id: Option<String>,
    aspect_ratio: Option<ProjectExtendedResponseAspectRatio>,
    agent_settings: Option<StudioAgentSettingsModel>,
    quality_preset: Option<QualityPresetType>,
    chapters: Option<Vec<ChapterResponse>>,
    pronunciation_dictionary_versions: Option<Vec<PronunciationDictionaryVersionResponseModel>>,
    pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocatorResponseModel>>,
    apply_text_normalization: Option<ProjectExtendedResponseApplyTextNormalization>,
    experimental: Option<HashMap<String, serde_json::Value>>,
    assets: Option<Vec<ProjectExtendedResponseAssetsItem>>,
    voices: Option<Vec<ProjectVoiceResponseModel>>,
    base_voices: Option<Vec<Voice>>,
    publishing_read: Option<DirectPublishingReadResponseModel>,
    default_title_voice_id: Option<String>,
    default_paragraph_voice_id: Option<String>,
}

impl ProjectExtendedResponseBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn create_date_unix(mut self, value: i64) -> Self {
        self.create_date_unix = Some(value);
        self
    }

    pub fn created_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_user_id = Some(value.into());
        self
    }

    pub fn default_title_voice_ref_id(mut self, value: impl Into<String>) -> Self {
        self.default_title_voice_ref_id = Some(value.into());
        self
    }

    pub fn default_paragraph_voice_ref_id(mut self, value: impl Into<String>) -> Self {
        self.default_paragraph_voice_ref_id = Some(value.into());
        self
    }

    pub fn default_model_id(mut self, value: impl Into<String>) -> Self {
        self.default_model_id = Some(value.into());
        self
    }

    pub fn last_conversion_date_unix(mut self, value: i64) -> Self {
        self.last_conversion_date_unix = Some(value);
        self
    }

    pub fn can_be_downloaded(mut self, value: bool) -> Self {
        self.can_be_downloaded = Some(value);
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

    pub fn cover_image_url(mut self, value: impl Into<String>) -> Self {
        self.cover_image_url = Some(value.into());
        self
    }

    pub fn target_audience(mut self, value: ProjectExtendedResponseTargetAudience) -> Self {
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

    pub fn volume_normalization(mut self, value: bool) -> Self {
        self.volume_normalization = Some(value);
        self
    }

    pub fn state(mut self, value: ProjectState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn access_level(mut self, value: ProjectExtendedResponseAccessLevel) -> Self {
        self.access_level = Some(value);
        self
    }

    pub fn fiction(mut self, value: ProjectExtendedResponseFiction) -> Self {
        self.fiction = Some(value);
        self
    }

    pub fn quality_check_on(mut self, value: bool) -> Self {
        self.quality_check_on = Some(value);
        self
    }

    pub fn quality_check_on_when_bulk_convert(mut self, value: bool) -> Self {
        self.quality_check_on_when_bulk_convert = Some(value);
        self
    }

    pub fn creation_meta(mut self, value: ProjectCreationMetaResponseModel) -> Self {
        self.creation_meta = Some(value);
        self
    }

    pub fn source_type(mut self, value: ProjectExtendedResponseSourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    pub fn chapters_enabled(mut self, value: bool) -> Self {
        self.chapters_enabled = Some(value);
        self
    }

    pub fn captions_enabled(mut self, value: bool) -> Self {
        self.captions_enabled = Some(value);
        self
    }

    pub fn caption_style(mut self, value: CaptionStyleModel) -> Self {
        self.caption_style = Some(value);
        self
    }

    pub fn caption_style_template_overrides(mut self, value: HashMap<String, Option<CaptionStyleModel>>) -> Self {
        self.caption_style_template_overrides = Some(value);
        self
    }

    pub fn public_share_id(mut self, value: impl Into<String>) -> Self {
        self.public_share_id = Some(value.into());
        self
    }

    pub fn aspect_ratio(mut self, value: ProjectExtendedResponseAspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn agent_settings(mut self, value: StudioAgentSettingsModel) -> Self {
        self.agent_settings = Some(value);
        self
    }

    pub fn quality_preset(mut self, value: QualityPresetType) -> Self {
        self.quality_preset = Some(value);
        self
    }

    pub fn chapters(mut self, value: Vec<ChapterResponse>) -> Self {
        self.chapters = Some(value);
        self
    }

    pub fn pronunciation_dictionary_versions(mut self, value: Vec<PronunciationDictionaryVersionResponseModel>) -> Self {
        self.pronunciation_dictionary_versions = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<PronunciationDictionaryLocatorResponseModel>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn apply_text_normalization(mut self, value: ProjectExtendedResponseApplyTextNormalization) -> Self {
        self.apply_text_normalization = Some(value);
        self
    }

    pub fn experimental(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.experimental = Some(value);
        self
    }

    pub fn assets(mut self, value: Vec<ProjectExtendedResponseAssetsItem>) -> Self {
        self.assets = Some(value);
        self
    }

    pub fn voices(mut self, value: Vec<ProjectVoiceResponseModel>) -> Self {
        self.voices = Some(value);
        self
    }

    pub fn base_voices(mut self, value: Vec<Voice>) -> Self {
        self.base_voices = Some(value);
        self
    }

    pub fn publishing_read(mut self, value: DirectPublishingReadResponseModel) -> Self {
        self.publishing_read = Some(value);
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

    /// Consumes the builder and constructs a [`ProjectExtendedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](ProjectExtendedResponseBuilder::project_id)
    /// - [`name`](ProjectExtendedResponseBuilder::name)
    /// - [`create_date_unix`](ProjectExtendedResponseBuilder::create_date_unix)
    /// - [`default_title_voice_ref_id`](ProjectExtendedResponseBuilder::default_title_voice_ref_id)
    /// - [`default_paragraph_voice_ref_id`](ProjectExtendedResponseBuilder::default_paragraph_voice_ref_id)
    /// - [`default_model_id`](ProjectExtendedResponseBuilder::default_model_id)
    /// - [`can_be_downloaded`](ProjectExtendedResponseBuilder::can_be_downloaded)
    /// - [`volume_normalization`](ProjectExtendedResponseBuilder::volume_normalization)
    /// - [`state`](ProjectExtendedResponseBuilder::state)
    /// - [`access_level`](ProjectExtendedResponseBuilder::access_level)
    /// - [`quality_check_on`](ProjectExtendedResponseBuilder::quality_check_on)
    /// - [`quality_check_on_when_bulk_convert`](ProjectExtendedResponseBuilder::quality_check_on_when_bulk_convert)
    /// - [`quality_preset`](ProjectExtendedResponseBuilder::quality_preset)
    /// - [`chapters`](ProjectExtendedResponseBuilder::chapters)
    /// - [`pronunciation_dictionary_versions`](ProjectExtendedResponseBuilder::pronunciation_dictionary_versions)
    /// - [`pronunciation_dictionary_locators`](ProjectExtendedResponseBuilder::pronunciation_dictionary_locators)
    /// - [`apply_text_normalization`](ProjectExtendedResponseBuilder::apply_text_normalization)
    /// - [`assets`](ProjectExtendedResponseBuilder::assets)
    /// - [`voices`](ProjectExtendedResponseBuilder::voices)
    pub fn build(self) -> Result<ProjectExtendedResponse, BuildError> {
        Ok(ProjectExtendedResponse {
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            create_date_unix: self.create_date_unix.ok_or_else(|| BuildError::missing_field("create_date_unix"))?,
            created_by_user_id: self.created_by_user_id,
            default_title_voice_ref_id: self.default_title_voice_ref_id.ok_or_else(|| BuildError::missing_field("default_title_voice_ref_id"))?,
            default_paragraph_voice_ref_id: self.default_paragraph_voice_ref_id.ok_or_else(|| BuildError::missing_field("default_paragraph_voice_ref_id"))?,
            default_model_id: self.default_model_id.ok_or_else(|| BuildError::missing_field("default_model_id"))?,
            last_conversion_date_unix: self.last_conversion_date_unix,
            can_be_downloaded: self.can_be_downloaded.ok_or_else(|| BuildError::missing_field("can_be_downloaded"))?,
            title: self.title,
            author: self.author,
            description: self.description,
            genres: self.genres,
            cover_image_url: self.cover_image_url,
            target_audience: self.target_audience,
            language: self.language,
            content_type: self.content_type,
            original_publication_date: self.original_publication_date,
            mature_content: self.mature_content,
            isbn_number: self.isbn_number,
            volume_normalization: self.volume_normalization.ok_or_else(|| BuildError::missing_field("volume_normalization"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            access_level: self.access_level.ok_or_else(|| BuildError::missing_field("access_level"))?,
            fiction: self.fiction,
            quality_check_on: self.quality_check_on.ok_or_else(|| BuildError::missing_field("quality_check_on"))?,
            quality_check_on_when_bulk_convert: self.quality_check_on_when_bulk_convert.ok_or_else(|| BuildError::missing_field("quality_check_on_when_bulk_convert"))?,
            creation_meta: self.creation_meta,
            source_type: self.source_type,
            chapters_enabled: self.chapters_enabled,
            captions_enabled: self.captions_enabled,
            caption_style: self.caption_style,
            caption_style_template_overrides: self.caption_style_template_overrides,
            public_share_id: self.public_share_id,
            aspect_ratio: self.aspect_ratio,
            agent_settings: self.agent_settings,
            quality_preset: self.quality_preset.ok_or_else(|| BuildError::missing_field("quality_preset"))?,
            chapters: self.chapters.ok_or_else(|| BuildError::missing_field("chapters"))?,
            pronunciation_dictionary_versions: self.pronunciation_dictionary_versions.ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_versions"))?,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators.ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_locators"))?,
            apply_text_normalization: self.apply_text_normalization.ok_or_else(|| BuildError::missing_field("apply_text_normalization"))?,
            experimental: self.experimental,
            assets: self.assets.ok_or_else(|| BuildError::missing_field("assets"))?,
            voices: self.voices.ok_or_else(|| BuildError::missing_field("voices"))?,
            base_voices: self.base_voices,
            publishing_read: self.publishing_read,
            default_title_voice_id: self.default_title_voice_id,
            default_paragraph_voice_id: self.default_paragraph_voice_id,
        })
    }
}
