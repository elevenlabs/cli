pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceSharingResponse {
    /// The status of the voice sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VoiceSharingState>,
    /// The sample ID of the history item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_item_sample_id: Option<String>,
    /// The date of the voice sharing in Unix time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_unix: Option<i64>,
    /// A list of whitelisted emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelisted_emails: Option<Vec<String>>,
    /// The ID of the public owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_owner_id: Option<String>,
    /// The ID of the original voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_voice_id: Option<String>,
    /// Whether financial rewards are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_rewards_enabled: Option<bool>,
    /// Whether free users are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_users_allowed: Option<bool>,
    /// Whether live moderation is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_moderation_enabled: Option<bool>,
    /// The rate of the voice sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub rate: Option<f64>,
    /// The rate of the voice sharing in USD per 1000 credits.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub fiat_rate: Option<f64>,
    /// The notice period of the voice sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice_period: Option<i64>,
    /// The date of the voice sharing in Unix time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_at_unix: Option<i64>,
    /// Whether voice mixing is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_mixing_allowed: Option<bool>,
    /// Whether the voice is featured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    /// The category of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<VoiceSharingResponseCategory>,
    /// Whether the reader app is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_app_enabled: Option<bool>,
    /// The image URL of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The ban reason of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_reason: Option<String>,
    /// The number of likes on the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked_by_count: Option<i64>,
    /// The number of clones on the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloned_by_count: Option<i64>,
    /// The name of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The labels of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// The review status of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<ReviewStatus>,
    /// The review message of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_message: Option<String>,
    /// Whether the voice is enabled in the library.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_in_library: Option<bool>,
    /// The Instagram username of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram_username: Option<String>,
    /// The Twitter/X username of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_username: Option<String>,
    /// The YouTube username of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_username: Option<String>,
    /// The TikTok username of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiktok_username: Option<String>,
    /// The moderation check of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_check: Option<VoiceSharingModerationCheckResponseModel>,
    /// The reader restricted on of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_restricted_on: Option<Vec<ReaderResourceResponseModel>>,
}

impl VoiceSharingResponse {
    pub fn builder() -> VoiceSharingResponseBuilder {
        <VoiceSharingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceSharingResponseBuilder {
    status: Option<VoiceSharingState>,
    history_item_sample_id: Option<String>,
    date_unix: Option<i64>,
    whitelisted_emails: Option<Vec<String>>,
    public_owner_id: Option<String>,
    original_voice_id: Option<String>,
    financial_rewards_enabled: Option<bool>,
    free_users_allowed: Option<bool>,
    live_moderation_enabled: Option<bool>,
    rate: Option<f64>,
    fiat_rate: Option<f64>,
    notice_period: Option<i64>,
    disable_at_unix: Option<i64>,
    voice_mixing_allowed: Option<bool>,
    featured: Option<bool>,
    category: Option<VoiceSharingResponseCategory>,
    reader_app_enabled: Option<bool>,
    image_url: Option<String>,
    ban_reason: Option<String>,
    liked_by_count: Option<i64>,
    cloned_by_count: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    labels: Option<HashMap<String, String>>,
    review_status: Option<ReviewStatus>,
    review_message: Option<String>,
    enabled_in_library: Option<bool>,
    instagram_username: Option<String>,
    twitter_username: Option<String>,
    youtube_username: Option<String>,
    tiktok_username: Option<String>,
    moderation_check: Option<VoiceSharingModerationCheckResponseModel>,
    reader_restricted_on: Option<Vec<ReaderResourceResponseModel>>,
}

impl VoiceSharingResponseBuilder {
    pub fn status(mut self, value: VoiceSharingState) -> Self {
        self.status = Some(value);
        self
    }

    pub fn history_item_sample_id(mut self, value: impl Into<String>) -> Self {
        self.history_item_sample_id = Some(value.into());
        self
    }

    pub fn date_unix(mut self, value: i64) -> Self {
        self.date_unix = Some(value);
        self
    }

    pub fn whitelisted_emails(mut self, value: Vec<String>) -> Self {
        self.whitelisted_emails = Some(value);
        self
    }

    pub fn public_owner_id(mut self, value: impl Into<String>) -> Self {
        self.public_owner_id = Some(value.into());
        self
    }

    pub fn original_voice_id(mut self, value: impl Into<String>) -> Self {
        self.original_voice_id = Some(value.into());
        self
    }

    pub fn financial_rewards_enabled(mut self, value: bool) -> Self {
        self.financial_rewards_enabled = Some(value);
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

    pub fn rate(mut self, value: f64) -> Self {
        self.rate = Some(value);
        self
    }

    pub fn fiat_rate(mut self, value: f64) -> Self {
        self.fiat_rate = Some(value);
        self
    }

    pub fn notice_period(mut self, value: i64) -> Self {
        self.notice_period = Some(value);
        self
    }

    pub fn disable_at_unix(mut self, value: i64) -> Self {
        self.disable_at_unix = Some(value);
        self
    }

    pub fn voice_mixing_allowed(mut self, value: bool) -> Self {
        self.voice_mixing_allowed = Some(value);
        self
    }

    pub fn featured(mut self, value: bool) -> Self {
        self.featured = Some(value);
        self
    }

    pub fn category(mut self, value: VoiceSharingResponseCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn reader_app_enabled(mut self, value: bool) -> Self {
        self.reader_app_enabled = Some(value);
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn ban_reason(mut self, value: impl Into<String>) -> Self {
        self.ban_reason = Some(value.into());
        self
    }

    pub fn liked_by_count(mut self, value: i64) -> Self {
        self.liked_by_count = Some(value);
        self
    }

    pub fn cloned_by_count(mut self, value: i64) -> Self {
        self.cloned_by_count = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn labels(mut self, value: HashMap<String, String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn review_status(mut self, value: ReviewStatus) -> Self {
        self.review_status = Some(value);
        self
    }

    pub fn review_message(mut self, value: impl Into<String>) -> Self {
        self.review_message = Some(value.into());
        self
    }

    pub fn enabled_in_library(mut self, value: bool) -> Self {
        self.enabled_in_library = Some(value);
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

    pub fn moderation_check(mut self, value: VoiceSharingModerationCheckResponseModel) -> Self {
        self.moderation_check = Some(value);
        self
    }

    pub fn reader_restricted_on(mut self, value: Vec<ReaderResourceResponseModel>) -> Self {
        self.reader_restricted_on = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceSharingResponse`].
    pub fn build(self) -> Result<VoiceSharingResponse, BuildError> {
        Ok(VoiceSharingResponse {
            status: self.status,
            history_item_sample_id: self.history_item_sample_id,
            date_unix: self.date_unix,
            whitelisted_emails: self.whitelisted_emails,
            public_owner_id: self.public_owner_id,
            original_voice_id: self.original_voice_id,
            financial_rewards_enabled: self.financial_rewards_enabled,
            free_users_allowed: self.free_users_allowed,
            live_moderation_enabled: self.live_moderation_enabled,
            rate: self.rate,
            fiat_rate: self.fiat_rate,
            notice_period: self.notice_period,
            disable_at_unix: self.disable_at_unix,
            voice_mixing_allowed: self.voice_mixing_allowed,
            featured: self.featured,
            category: self.category,
            reader_app_enabled: self.reader_app_enabled,
            image_url: self.image_url,
            ban_reason: self.ban_reason,
            liked_by_count: self.liked_by_count,
            cloned_by_count: self.cloned_by_count,
            name: self.name,
            description: self.description,
            labels: self.labels,
            review_status: self.review_status,
            review_message: self.review_message,
            enabled_in_library: self.enabled_in_library,
            instagram_username: self.instagram_username,
            twitter_username: self.twitter_username,
            youtube_username: self.youtube_username,
            tiktok_username: self.tiktok_username,
            moderation_check: self.moderation_check,
            reader_restricted_on: self.reader_restricted_on,
        })
    }
}
