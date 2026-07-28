pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LibraryVoiceResponse {
    /// The public owner id of the voice.
    #[serde(default)]
    pub public_owner_id: String,
    /// The id of the voice.
    #[serde(default)]
    pub voice_id: String,
    /// The date the voice was added to the library in Unix time.
    #[serde(default)]
    pub date_unix: i64,
    /// The name of the voice.
    #[serde(default)]
    pub name: String,
    /// The accent of the voice.
    #[serde(default)]
    pub accent: String,
    /// The gender of the voice.
    #[serde(default)]
    pub gender: String,
    /// The age of the voice.
    #[serde(default)]
    pub age: String,
    /// The descriptive of the voice.
    #[serde(default)]
    pub descriptive: String,
    /// The use case of the voice.
    #[serde(default)]
    pub use_case: String,
    /// The category of the voice.
    pub category: LibraryVoiceResponseCategory,
    /// The language of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The locale of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// The description of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The preview URL of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
    /// The usage character count of the voice in the last year.
    #[serde(rename = "usage_character_count_1y")]
    #[serde(default)]
    pub usage_character_count1y: i64,
    /// The usage character count of the voice in the last 7 days.
    #[serde(rename = "usage_character_count_7d")]
    #[serde(default)]
    pub usage_character_count7d: i64,
    /// The play API usage character count of the voice in the last year.
    #[serde(rename = "play_api_usage_character_count_1y")]
    #[serde(default)]
    pub play_api_usage_character_count1y: i64,
    /// The number of times the voice has been cloned.
    #[serde(default)]
    pub cloned_by_count: i64,
    /// The rate multiplier of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub rate: Option<f64>,
    /// The rate of the voice in USD per 1000 credits. null if default
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub fiat_rate: Option<f64>,
    /// Whether free users are allowed to use the voice.
    #[serde(default)]
    pub free_users_allowed: bool,
    /// Whether live moderation is enabled for the voice.
    #[serde(default)]
    pub live_moderation_enabled: bool,
    /// Whether the voice is featured.
    #[serde(default)]
    pub featured: bool,
    /// The verified languages of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_languages: Option<Vec<VerifiedVoiceLanguageResponseModel>>,
    /// The notice period of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_period: Option<i64>,
    /// The Instagram username of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram_username: Option<String>,
    /// The Twitter username of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_username: Option<String>,
    /// The YouTube username of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_username: Option<String>,
    /// The TikTok username of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiktok_username: Option<String>,
    /// The image URL of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Whether the voice was added by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_added_by_user: Option<bool>,
    /// Whether the voice is bookmarked by the current user. Only relevant when is_added_by_user is True.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bookmarked: Option<bool>,
}

impl LibraryVoiceResponse {
    pub fn builder() -> LibraryVoiceResponseBuilder {
        <LibraryVoiceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LibraryVoiceResponseBuilder {
    public_owner_id: Option<String>,
    voice_id: Option<String>,
    date_unix: Option<i64>,
    name: Option<String>,
    accent: Option<String>,
    gender: Option<String>,
    age: Option<String>,
    descriptive: Option<String>,
    use_case: Option<String>,
    category: Option<LibraryVoiceResponseCategory>,
    language: Option<String>,
    locale: Option<String>,
    description: Option<String>,
    preview_url: Option<String>,
    usage_character_count1y: Option<i64>,
    usage_character_count7d: Option<i64>,
    play_api_usage_character_count1y: Option<i64>,
    cloned_by_count: Option<i64>,
    rate: Option<f64>,
    fiat_rate: Option<f64>,
    free_users_allowed: Option<bool>,
    live_moderation_enabled: Option<bool>,
    featured: Option<bool>,
    verified_languages: Option<Vec<VerifiedVoiceLanguageResponseModel>>,
    notice_period: Option<i64>,
    instagram_username: Option<String>,
    twitter_username: Option<String>,
    youtube_username: Option<String>,
    tiktok_username: Option<String>,
    image_url: Option<String>,
    is_added_by_user: Option<bool>,
    is_bookmarked: Option<bool>,
}

impl LibraryVoiceResponseBuilder {
    pub fn public_owner_id(mut self, value: impl Into<String>) -> Self {
        self.public_owner_id = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn date_unix(mut self, value: i64) -> Self {
        self.date_unix = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn accent(mut self, value: impl Into<String>) -> Self {
        self.accent = Some(value.into());
        self
    }

    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(value.into());
        self
    }

    pub fn age(mut self, value: impl Into<String>) -> Self {
        self.age = Some(value.into());
        self
    }

    pub fn descriptive(mut self, value: impl Into<String>) -> Self {
        self.descriptive = Some(value.into());
        self
    }

    pub fn use_case(mut self, value: impl Into<String>) -> Self {
        self.use_case = Some(value.into());
        self
    }

    pub fn category(mut self, value: LibraryVoiceResponseCategory) -> Self {
        self.category = Some(value);
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

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn preview_url(mut self, value: impl Into<String>) -> Self {
        self.preview_url = Some(value.into());
        self
    }

    pub fn usage_character_count1y(mut self, value: i64) -> Self {
        self.usage_character_count1y = Some(value);
        self
    }

    pub fn usage_character_count7d(mut self, value: i64) -> Self {
        self.usage_character_count7d = Some(value);
        self
    }

    pub fn play_api_usage_character_count1y(mut self, value: i64) -> Self {
        self.play_api_usage_character_count1y = Some(value);
        self
    }

    pub fn cloned_by_count(mut self, value: i64) -> Self {
        self.cloned_by_count = Some(value);
        self
    }

    pub fn rate(mut self, value: f64) -> Self {
        self.rate = Some(value);
        self
    }

    pub fn fiat_rate(mut self, value: f64) -> Self {
        self.fiat_rate = Some(value);
        self
    }

    pub fn free_users_allowed(mut self, value: bool) -> Self {
        self.free_users_allowed = Some(value);
        self
    }

    pub fn live_moderation_enabled(mut self, value: bool) -> Self {
        self.live_moderation_enabled = Some(value);
        self
    }

    pub fn featured(mut self, value: bool) -> Self {
        self.featured = Some(value);
        self
    }

    pub fn verified_languages(mut self, value: Vec<VerifiedVoiceLanguageResponseModel>) -> Self {
        self.verified_languages = Some(value);
        self
    }

    pub fn notice_period(mut self, value: i64) -> Self {
        self.notice_period = Some(value);
        self
    }

    pub fn instagram_username(mut self, value: impl Into<String>) -> Self {
        self.instagram_username = Some(value.into());
        self
    }

    pub fn twitter_username(mut self, value: impl Into<String>) -> Self {
        self.twitter_username = Some(value.into());
        self
    }

    pub fn youtube_username(mut self, value: impl Into<String>) -> Self {
        self.youtube_username = Some(value.into());
        self
    }

    pub fn tiktok_username(mut self, value: impl Into<String>) -> Self {
        self.tiktok_username = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn is_added_by_user(mut self, value: bool) -> Self {
        self.is_added_by_user = Some(value);
        self
    }

    pub fn is_bookmarked(mut self, value: bool) -> Self {
        self.is_bookmarked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LibraryVoiceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`public_owner_id`](LibraryVoiceResponseBuilder::public_owner_id)
    /// - [`voice_id`](LibraryVoiceResponseBuilder::voice_id)
    /// - [`date_unix`](LibraryVoiceResponseBuilder::date_unix)
    /// - [`name`](LibraryVoiceResponseBuilder::name)
    /// - [`accent`](LibraryVoiceResponseBuilder::accent)
    /// - [`gender`](LibraryVoiceResponseBuilder::gender)
    /// - [`age`](LibraryVoiceResponseBuilder::age)
    /// - [`descriptive`](LibraryVoiceResponseBuilder::descriptive)
    /// - [`use_case`](LibraryVoiceResponseBuilder::use_case)
    /// - [`category`](LibraryVoiceResponseBuilder::category)
    /// - [`usage_character_count1y`](LibraryVoiceResponseBuilder::usage_character_count1y)
    /// - [`usage_character_count7d`](LibraryVoiceResponseBuilder::usage_character_count7d)
    /// - [`play_api_usage_character_count1y`](LibraryVoiceResponseBuilder::play_api_usage_character_count1y)
    /// - [`cloned_by_count`](LibraryVoiceResponseBuilder::cloned_by_count)
    /// - [`free_users_allowed`](LibraryVoiceResponseBuilder::free_users_allowed)
    /// - [`live_moderation_enabled`](LibraryVoiceResponseBuilder::live_moderation_enabled)
    /// - [`featured`](LibraryVoiceResponseBuilder::featured)
    pub fn build(self) -> Result<LibraryVoiceResponse, BuildError> {
        Ok(LibraryVoiceResponse {
            public_owner_id: self.public_owner_id.ok_or_else(|| BuildError::missing_field("public_owner_id"))?,
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            date_unix: self.date_unix.ok_or_else(|| BuildError::missing_field("date_unix"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            accent: self.accent.ok_or_else(|| BuildError::missing_field("accent"))?,
            gender: self.gender.ok_or_else(|| BuildError::missing_field("gender"))?,
            age: self.age.ok_or_else(|| BuildError::missing_field("age"))?,
            descriptive: self.descriptive.ok_or_else(|| BuildError::missing_field("descriptive"))?,
            use_case: self.use_case.ok_or_else(|| BuildError::missing_field("use_case"))?,
            category: self.category.ok_or_else(|| BuildError::missing_field("category"))?,
            language: self.language,
            locale: self.locale,
            description: self.description,
            preview_url: self.preview_url,
            usage_character_count1y: self.usage_character_count1y.ok_or_else(|| BuildError::missing_field("usage_character_count1y"))?,
            usage_character_count7d: self.usage_character_count7d.ok_or_else(|| BuildError::missing_field("usage_character_count7d"))?,
            play_api_usage_character_count1y: self.play_api_usage_character_count1y.ok_or_else(|| BuildError::missing_field("play_api_usage_character_count1y"))?,
            cloned_by_count: self.cloned_by_count.ok_or_else(|| BuildError::missing_field("cloned_by_count"))?,
            rate: self.rate,
            fiat_rate: self.fiat_rate,
            free_users_allowed: self.free_users_allowed.ok_or_else(|| BuildError::missing_field("free_users_allowed"))?,
            live_moderation_enabled: self.live_moderation_enabled.ok_or_else(|| BuildError::missing_field("live_moderation_enabled"))?,
            featured: self.featured.ok_or_else(|| BuildError::missing_field("featured"))?,
            verified_languages: self.verified_languages,
            notice_period: self.notice_period,
            instagram_username: self.instagram_username,
            twitter_username: self.twitter_username,
            youtube_username: self.youtube_username,
            tiktok_username: self.tiktok_username,
            image_url: self.image_url,
            is_added_by_user: self.is_added_by_user,
            is_bookmarked: self.is_bookmarked,
        })
    }
}
