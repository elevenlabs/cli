pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DirectPublishingReadResponseModel {
    #[serde(default)]
    pub read_id: String,
    #[serde(default)]
    pub created_at_unix: i64,
    #[serde(default)]
    pub updated_at_unix: i64,
    #[serde(default)]
    pub word_count: i64,
    #[serde(default)]
    pub char_count: i64,
    #[serde(default)]
    pub chapters: Vec<ReadMetadataChapterDbModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<DirectPublishingReadResponseModelDisplayMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<Vec<DirectPublishingReadResponseModelGenreItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<DirectPublishingReadResponseModelTargetAudience>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mature_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safesearch_adult: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ean: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_terms: Option<ReadLegalTerms>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_guidelines_terms: Option<ReadLegalTerms>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_from_project_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishing_project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publishing_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_profile_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_score: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_territories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<Contributor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_type: Option<DirectPublishingReadResponseModelPayoutType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub list_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_audio_project_export_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_audio_document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_audio_object: Option<PreviewAudioDbModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_config: Option<SampleConfigDbModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<ReviewResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_use_assistant: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_voice_changer_on: Option<bool>,
}

impl DirectPublishingReadResponseModel {
    pub fn builder() -> DirectPublishingReadResponseModelBuilder {
        <DirectPublishingReadResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DirectPublishingReadResponseModelBuilder {
    read_id: Option<String>,
    created_at_unix: Option<i64>,
    updated_at_unix: Option<i64>,
    word_count: Option<i64>,
    char_count: Option<i64>,
    chapters: Option<Vec<ReadMetadataChapterDbModel>>,
    title: Option<String>,
    author: Option<String>,
    description: Option<String>,
    article_image_url: Option<String>,
    language: Option<String>,
    locale: Option<String>,
    display_mode: Option<DirectPublishingReadResponseModelDisplayMode>,
    genre: Option<Vec<DirectPublishingReadResponseModelGenreItem>>,
    fiction: Option<String>,
    content_type: Option<String>,
    original_file_type: Option<String>,
    target_audience: Option<DirectPublishingReadResponseModelTargetAudience>,
    mature_content: Option<bool>,
    safesearch_adult: Option<bool>,
    origin: Option<String>,
    publication_date: Option<String>,
    isbn: Option<String>,
    ean: Option<String>,
    legal_terms: Option<ReadLegalTerms>,
    content_guidelines_terms: Option<ReadLegalTerms>,
    last_updated_from_project_unix: Option<i64>,
    publishing_project_id: Option<String>,
    publishing_state: Option<String>,
    publisher_profile_id: Option<String>,
    quality_score: Option<i64>,
    publisher: Option<String>,
    copyright: Option<String>,
    subtitle: Option<String>,
    distribution_territories: Option<Vec<String>>,
    edition: Option<String>,
    contributors: Option<Vec<Contributor>>,
    payout_type: Option<DirectPublishingReadResponseModelPayoutType>,
    list_price: Option<f64>,
    currency: Option<String>,
    original_audio_project_export_id: Option<String>,
    original_audio_document_id: Option<String>,
    series_id: Option<String>,
    volume: Option<i64>,
    published_at_unix: Option<i64>,
    read_slug: Option<String>,
    preview_audio_object: Option<PreviewAudioDbModel>,
    sample_config: Option<SampleConfigDbModel>,
    review: Option<ReviewResponseModel>,
    voice_id: Option<String>,
    can_use_assistant: Option<bool>,
    is_voice_changer_on: Option<bool>,
}

impl DirectPublishingReadResponseModelBuilder {
    pub fn read_id(mut self, value: impl Into<String>) -> Self {
        self.read_id = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn updated_at_unix(mut self, value: i64) -> Self {
        self.updated_at_unix = Some(value);
        self
    }

    pub fn word_count(mut self, value: i64) -> Self {
        self.word_count = Some(value);
        self
    }

    pub fn char_count(mut self, value: i64) -> Self {
        self.char_count = Some(value);
        self
    }

    pub fn chapters(mut self, value: Vec<ReadMetadataChapterDbModel>) -> Self {
        self.chapters = Some(value);
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

    pub fn article_image_url(mut self, value: impl Into<String>) -> Self {
        self.article_image_url = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
        self
    }

    pub fn display_mode(mut self, value: DirectPublishingReadResponseModelDisplayMode) -> Self {
        self.display_mode = Some(value);
        self
    }

    pub fn genre(mut self, value: Vec<DirectPublishingReadResponseModelGenreItem>) -> Self {
        self.genre = Some(value);
        self
    }

    pub fn fiction(mut self, value: impl Into<String>) -> Self {
        self.fiction = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn original_file_type(mut self, value: impl Into<String>) -> Self {
        self.original_file_type = Some(value.into());
        self
    }

    pub fn target_audience(mut self, value: DirectPublishingReadResponseModelTargetAudience) -> Self {
        self.target_audience = Some(value);
        self
    }

    pub fn mature_content(mut self, value: bool) -> Self {
        self.mature_content = Some(value);
        self
    }

    pub fn safesearch_adult(mut self, value: bool) -> Self {
        self.safesearch_adult = Some(value);
        self
    }

    pub fn origin(mut self, value: impl Into<String>) -> Self {
        self.origin = Some(value.into());
        self
    }

    pub fn publication_date(mut self, value: impl Into<String>) -> Self {
        self.publication_date = Some(value.into());
        self
    }

    pub fn isbn(mut self, value: impl Into<String>) -> Self {
        self.isbn = Some(value.into());
        self
    }

    pub fn ean(mut self, value: impl Into<String>) -> Self {
        self.ean = Some(value.into());
        self
    }

    pub fn legal_terms(mut self, value: ReadLegalTerms) -> Self {
        self.legal_terms = Some(value);
        self
    }

    pub fn content_guidelines_terms(mut self, value: ReadLegalTerms) -> Self {
        self.content_guidelines_terms = Some(value);
        self
    }

    pub fn last_updated_from_project_unix(mut self, value: i64) -> Self {
        self.last_updated_from_project_unix = Some(value);
        self
    }

    pub fn publishing_project_id(mut self, value: impl Into<String>) -> Self {
        self.publishing_project_id = Some(value.into());
        self
    }

    pub fn publishing_state(mut self, value: impl Into<String>) -> Self {
        self.publishing_state = Some(value.into());
        self
    }

    pub fn publisher_profile_id(mut self, value: impl Into<String>) -> Self {
        self.publisher_profile_id = Some(value.into());
        self
    }

    pub fn quality_score(mut self, value: i64) -> Self {
        self.quality_score = Some(value);
        self
    }

    pub fn publisher(mut self, value: impl Into<String>) -> Self {
        self.publisher = Some(value.into());
        self
    }

    pub fn copyright(mut self, value: impl Into<String>) -> Self {
        self.copyright = Some(value.into());
        self
    }

    pub fn subtitle(mut self, value: impl Into<String>) -> Self {
        self.subtitle = Some(value.into());
        self
    }

    pub fn distribution_territories(mut self, value: Vec<String>) -> Self {
        self.distribution_territories = Some(value);
        self
    }

    pub fn edition(mut self, value: impl Into<String>) -> Self {
        self.edition = Some(value.into());
        self
    }

    pub fn contributors(mut self, value: Vec<Contributor>) -> Self {
        self.contributors = Some(value);
        self
    }

    pub fn payout_type(mut self, value: DirectPublishingReadResponseModelPayoutType) -> Self {
        self.payout_type = Some(value);
        self
    }

    pub fn list_price(mut self, value: f64) -> Self {
        self.list_price = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn original_audio_project_export_id(mut self, value: impl Into<String>) -> Self {
        self.original_audio_project_export_id = Some(value.into());
        self
    }

    pub fn original_audio_document_id(mut self, value: impl Into<String>) -> Self {
        self.original_audio_document_id = Some(value.into());
        self
    }

    pub fn series_id(mut self, value: impl Into<String>) -> Self {
        self.series_id = Some(value.into());
        self
    }

    pub fn volume(mut self, value: i64) -> Self {
        self.volume = Some(value);
        self
    }

    pub fn published_at_unix(mut self, value: i64) -> Self {
        self.published_at_unix = Some(value);
        self
    }

    pub fn read_slug(mut self, value: impl Into<String>) -> Self {
        self.read_slug = Some(value.into());
        self
    }

    pub fn preview_audio_object(mut self, value: PreviewAudioDbModel) -> Self {
        self.preview_audio_object = Some(value);
        self
    }

    pub fn sample_config(mut self, value: SampleConfigDbModel) -> Self {
        self.sample_config = Some(value);
        self
    }

    pub fn review(mut self, value: ReviewResponseModel) -> Self {
        self.review = Some(value);
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn can_use_assistant(mut self, value: bool) -> Self {
        self.can_use_assistant = Some(value);
        self
    }

    pub fn is_voice_changer_on(mut self, value: bool) -> Self {
        self.is_voice_changer_on = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DirectPublishingReadResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`read_id`](DirectPublishingReadResponseModelBuilder::read_id)
    /// - [`created_at_unix`](DirectPublishingReadResponseModelBuilder::created_at_unix)
    /// - [`updated_at_unix`](DirectPublishingReadResponseModelBuilder::updated_at_unix)
    /// - [`word_count`](DirectPublishingReadResponseModelBuilder::word_count)
    /// - [`char_count`](DirectPublishingReadResponseModelBuilder::char_count)
    /// - [`chapters`](DirectPublishingReadResponseModelBuilder::chapters)
    pub fn build(self) -> Result<DirectPublishingReadResponseModel, BuildError> {
        Ok(DirectPublishingReadResponseModel {
            read_id: self.read_id.ok_or_else(|| BuildError::missing_field("read_id"))?,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            updated_at_unix: self.updated_at_unix.ok_or_else(|| BuildError::missing_field("updated_at_unix"))?,
            word_count: self.word_count.ok_or_else(|| BuildError::missing_field("word_count"))?,
            char_count: self.char_count.ok_or_else(|| BuildError::missing_field("char_count"))?,
            chapters: self.chapters.ok_or_else(|| BuildError::missing_field("chapters"))?,
            title: self.title,
            author: self.author,
            description: self.description,
            article_image_url: self.article_image_url,
            language: self.language,
            locale: self.locale,
            display_mode: self.display_mode,
            genre: self.genre,
            fiction: self.fiction,
            content_type: self.content_type,
            original_file_type: self.original_file_type,
            target_audience: self.target_audience,
            mature_content: self.mature_content,
            safesearch_adult: self.safesearch_adult,
            origin: self.origin,
            publication_date: self.publication_date,
            isbn: self.isbn,
            ean: self.ean,
            legal_terms: self.legal_terms,
            content_guidelines_terms: self.content_guidelines_terms,
            last_updated_from_project_unix: self.last_updated_from_project_unix,
            publishing_project_id: self.publishing_project_id,
            publishing_state: self.publishing_state,
            publisher_profile_id: self.publisher_profile_id,
            quality_score: self.quality_score,
            publisher: self.publisher,
            copyright: self.copyright,
            subtitle: self.subtitle,
            distribution_territories: self.distribution_territories,
            edition: self.edition,
            contributors: self.contributors,
            payout_type: self.payout_type,
            list_price: self.list_price,
            currency: self.currency,
            original_audio_project_export_id: self.original_audio_project_export_id,
            original_audio_document_id: self.original_audio_document_id,
            series_id: self.series_id,
            volume: self.volume,
            published_at_unix: self.published_at_unix,
            read_slug: self.read_slug,
            preview_audio_object: self.preview_audio_object,
            sample_config: self.sample_config,
            review: self.review,
            voice_id: self.voice_id,
            can_use_assistant: self.can_use_assistant,
            is_voice_changer_on: self.is_voice_changer_on,
        })
    }
}
