pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionExtrasResponseModel {
    /// The concurrency of the user.
    #[serde(default)]
    pub concurrency: i64,
    /// The Convai concurrency of the user.
    #[serde(default)]
    pub convai_concurrency: i64,
    /// The Music concurrency of the user on enterprise plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_music_concurrency: Option<i64>,
    /// Custom total finetunes limit for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_finetunes_total_limit: Option<i64>,
    /// Custom monthly finetunes limit for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_finetunes_monthly_limit: Option<i64>,
    /// Custom finetunes concurrency limit for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_finetunes_concurrency_limit: Option<i64>,
    /// The Convai characters per minute of the user. This field is deprecated and will always return None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convai_chars_per_minute: Option<i64>,
    /// The Convai ASR characters per minute of the user. This field is deprecated and will always return None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convai_asr_chars_per_minute: Option<i64>,
    /// Whether the user's logging is disabled.
    #[serde(default)]
    pub force_logging_disabled: bool,
    /// Whether the user can request manual pro voice verification.
    #[serde(default)]
    pub can_request_manual_pro_voice_verification: bool,
    /// Whether the user can bypass the voice captcha.
    #[serde(default)]
    pub can_bypass_voice_captcha: bool,
    /// The moderation status of the user.
    #[serde(default)]
    pub moderation: ModerationStatusResponseModel,
    /// The unused characters rolled over from the previous period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_characters_rolled_over_from_previous_period: Option<i64>,
    /// The overused characters rolled over from the previous period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overused_characters_rolled_over_from_previous_period: Option<i64>,
    /// Data on how the subscription is being used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<SubscriptionUsageResponseModel>,
}

impl SubscriptionExtrasResponseModel {
    pub fn builder() -> SubscriptionExtrasResponseModelBuilder {
        <SubscriptionExtrasResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionExtrasResponseModelBuilder {
    concurrency: Option<i64>,
    convai_concurrency: Option<i64>,
    enterprise_music_concurrency: Option<i64>,
    music_finetunes_total_limit: Option<i64>,
    music_finetunes_monthly_limit: Option<i64>,
    music_finetunes_concurrency_limit: Option<i64>,
    convai_chars_per_minute: Option<i64>,
    convai_asr_chars_per_minute: Option<i64>,
    force_logging_disabled: Option<bool>,
    can_request_manual_pro_voice_verification: Option<bool>,
    can_bypass_voice_captcha: Option<bool>,
    moderation: Option<ModerationStatusResponseModel>,
    unused_characters_rolled_over_from_previous_period: Option<i64>,
    overused_characters_rolled_over_from_previous_period: Option<i64>,
    usage: Option<SubscriptionUsageResponseModel>,
}

impl SubscriptionExtrasResponseModelBuilder {
    pub fn concurrency(mut self, value: i64) -> Self {
        self.concurrency = Some(value);
        self
    }

    pub fn convai_concurrency(mut self, value: i64) -> Self {
        self.convai_concurrency = Some(value);
        self
    }

    pub fn enterprise_music_concurrency(mut self, value: i64) -> Self {
        self.enterprise_music_concurrency = Some(value);
        self
    }

    pub fn music_finetunes_total_limit(mut self, value: i64) -> Self {
        self.music_finetunes_total_limit = Some(value);
        self
    }

    pub fn music_finetunes_monthly_limit(mut self, value: i64) -> Self {
        self.music_finetunes_monthly_limit = Some(value);
        self
    }

    pub fn music_finetunes_concurrency_limit(mut self, value: i64) -> Self {
        self.music_finetunes_concurrency_limit = Some(value);
        self
    }

    pub fn convai_chars_per_minute(mut self, value: i64) -> Self {
        self.convai_chars_per_minute = Some(value);
        self
    }

    pub fn convai_asr_chars_per_minute(mut self, value: i64) -> Self {
        self.convai_asr_chars_per_minute = Some(value);
        self
    }

    pub fn force_logging_disabled(mut self, value: bool) -> Self {
        self.force_logging_disabled = Some(value);
        self
    }

    pub fn can_request_manual_pro_voice_verification(mut self, value: bool) -> Self {
        self.can_request_manual_pro_voice_verification = Some(value);
        self
    }

    pub fn can_bypass_voice_captcha(mut self, value: bool) -> Self {
        self.can_bypass_voice_captcha = Some(value);
        self
    }

    pub fn moderation(mut self, value: ModerationStatusResponseModel) -> Self {
        self.moderation = Some(value);
        self
    }

    pub fn unused_characters_rolled_over_from_previous_period(mut self, value: i64) -> Self {
        self.unused_characters_rolled_over_from_previous_period = Some(value);
        self
    }

    pub fn overused_characters_rolled_over_from_previous_period(mut self, value: i64) -> Self {
        self.overused_characters_rolled_over_from_previous_period = Some(value);
        self
    }

    pub fn usage(mut self, value: SubscriptionUsageResponseModel) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionExtrasResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`concurrency`](SubscriptionExtrasResponseModelBuilder::concurrency)
    /// - [`convai_concurrency`](SubscriptionExtrasResponseModelBuilder::convai_concurrency)
    /// - [`force_logging_disabled`](SubscriptionExtrasResponseModelBuilder::force_logging_disabled)
    /// - [`can_request_manual_pro_voice_verification`](SubscriptionExtrasResponseModelBuilder::can_request_manual_pro_voice_verification)
    /// - [`can_bypass_voice_captcha`](SubscriptionExtrasResponseModelBuilder::can_bypass_voice_captcha)
    /// - [`moderation`](SubscriptionExtrasResponseModelBuilder::moderation)
    pub fn build(self) -> Result<SubscriptionExtrasResponseModel, BuildError> {
        Ok(SubscriptionExtrasResponseModel {
            concurrency: self.concurrency.ok_or_else(|| BuildError::missing_field("concurrency"))?,
            convai_concurrency: self.convai_concurrency.ok_or_else(|| BuildError::missing_field("convai_concurrency"))?,
            enterprise_music_concurrency: self.enterprise_music_concurrency,
            music_finetunes_total_limit: self.music_finetunes_total_limit,
            music_finetunes_monthly_limit: self.music_finetunes_monthly_limit,
            music_finetunes_concurrency_limit: self.music_finetunes_concurrency_limit,
            convai_chars_per_minute: self.convai_chars_per_minute,
            convai_asr_chars_per_minute: self.convai_asr_chars_per_minute,
            force_logging_disabled: self.force_logging_disabled.ok_or_else(|| BuildError::missing_field("force_logging_disabled"))?,
            can_request_manual_pro_voice_verification: self.can_request_manual_pro_voice_verification.ok_or_else(|| BuildError::missing_field("can_request_manual_pro_voice_verification"))?,
            can_bypass_voice_captcha: self.can_bypass_voice_captcha.ok_or_else(|| BuildError::missing_field("can_bypass_voice_captcha"))?,
            moderation: self.moderation.ok_or_else(|| BuildError::missing_field("moderation"))?,
            unused_characters_rolled_over_from_previous_period: self.unused_characters_rolled_over_from_previous_period,
            overused_characters_rolled_over_from_previous_period: self.overused_characters_rolled_over_from_previous_period,
            usage: self.usage,
        })
    }
}
