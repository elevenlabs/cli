pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ModerationStatusResponseModel {
    /// Whether the user is in probation.
    #[serde(default)]
    pub is_in_probation: bool,
    /// Whether the user's enterprise check nogo voice is enabled.
    #[serde(default)]
    pub enterprise_check_nogo_voice: bool,
    /// Whether the user's enterprise check block nogo voice is enabled.
    #[serde(default)]
    pub enterprise_check_block_nogo_voice: bool,
    /// Whether the user's never live moderate is enabled.
    #[serde(default)]
    pub never_live_moderate: bool,
    /// The number of similar voice uploads that have been blocked.
    #[serde(default)]
    pub nogo_voice_similar_voice_upload_count: i64,
    /// Whether the user's enterprise background moderation is enabled.
    #[serde(default)]
    pub enterprise_background_moderation_enabled: bool,
    /// Whether captcha is required when creating IVCs.
    #[serde(default)]
    pub is_ivc_captcha_required: bool,
    /// The safety status of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_status: Option<ModerationStatusResponseModelSafetyStatus>,
    /// The warning status of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_status: Option<ModerationStatusResponseModelWarningStatus>,
    /// Whether the user is on the watchlist.
    #[serde(default)]
    pub on_watchlist: bool,
}

impl ModerationStatusResponseModel {
    pub fn builder() -> ModerationStatusResponseModelBuilder {
        <ModerationStatusResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModerationStatusResponseModelBuilder {
    is_in_probation: Option<bool>,
    enterprise_check_nogo_voice: Option<bool>,
    enterprise_check_block_nogo_voice: Option<bool>,
    never_live_moderate: Option<bool>,
    nogo_voice_similar_voice_upload_count: Option<i64>,
    enterprise_background_moderation_enabled: Option<bool>,
    is_ivc_captcha_required: Option<bool>,
    safety_status: Option<ModerationStatusResponseModelSafetyStatus>,
    warning_status: Option<ModerationStatusResponseModelWarningStatus>,
    on_watchlist: Option<bool>,
}

impl ModerationStatusResponseModelBuilder {
    pub fn is_in_probation(mut self, value: bool) -> Self {
        self.is_in_probation = Some(value);
        self
    }

    pub fn enterprise_check_nogo_voice(mut self, value: bool) -> Self {
        self.enterprise_check_nogo_voice = Some(value);
        self
    }

    pub fn enterprise_check_block_nogo_voice(mut self, value: bool) -> Self {
        self.enterprise_check_block_nogo_voice = Some(value);
        self
    }

    pub fn never_live_moderate(mut self, value: bool) -> Self {
        self.never_live_moderate = Some(value);
        self
    }

    pub fn nogo_voice_similar_voice_upload_count(mut self, value: i64) -> Self {
        self.nogo_voice_similar_voice_upload_count = Some(value);
        self
    }

    pub fn enterprise_background_moderation_enabled(mut self, value: bool) -> Self {
        self.enterprise_background_moderation_enabled = Some(value);
        self
    }

    pub fn is_ivc_captcha_required(mut self, value: bool) -> Self {
        self.is_ivc_captcha_required = Some(value);
        self
    }

    pub fn safety_status(mut self, value: ModerationStatusResponseModelSafetyStatus) -> Self {
        self.safety_status = Some(value);
        self
    }

    pub fn warning_status(mut self, value: ModerationStatusResponseModelWarningStatus) -> Self {
        self.warning_status = Some(value);
        self
    }

    pub fn on_watchlist(mut self, value: bool) -> Self {
        self.on_watchlist = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModerationStatusResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_in_probation`](ModerationStatusResponseModelBuilder::is_in_probation)
    /// - [`enterprise_check_nogo_voice`](ModerationStatusResponseModelBuilder::enterprise_check_nogo_voice)
    /// - [`enterprise_check_block_nogo_voice`](ModerationStatusResponseModelBuilder::enterprise_check_block_nogo_voice)
    /// - [`never_live_moderate`](ModerationStatusResponseModelBuilder::never_live_moderate)
    /// - [`nogo_voice_similar_voice_upload_count`](ModerationStatusResponseModelBuilder::nogo_voice_similar_voice_upload_count)
    /// - [`enterprise_background_moderation_enabled`](ModerationStatusResponseModelBuilder::enterprise_background_moderation_enabled)
    /// - [`is_ivc_captcha_required`](ModerationStatusResponseModelBuilder::is_ivc_captcha_required)
    /// - [`on_watchlist`](ModerationStatusResponseModelBuilder::on_watchlist)
    pub fn build(self) -> Result<ModerationStatusResponseModel, BuildError> {
        Ok(ModerationStatusResponseModel {
            is_in_probation: self.is_in_probation.ok_or_else(|| BuildError::missing_field("is_in_probation"))?,
            enterprise_check_nogo_voice: self.enterprise_check_nogo_voice.ok_or_else(|| BuildError::missing_field("enterprise_check_nogo_voice"))?,
            enterprise_check_block_nogo_voice: self.enterprise_check_block_nogo_voice.ok_or_else(|| BuildError::missing_field("enterprise_check_block_nogo_voice"))?,
            never_live_moderate: self.never_live_moderate.ok_or_else(|| BuildError::missing_field("never_live_moderate"))?,
            nogo_voice_similar_voice_upload_count: self.nogo_voice_similar_voice_upload_count.ok_or_else(|| BuildError::missing_field("nogo_voice_similar_voice_upload_count"))?,
            enterprise_background_moderation_enabled: self.enterprise_background_moderation_enabled.ok_or_else(|| BuildError::missing_field("enterprise_background_moderation_enabled"))?,
            is_ivc_captcha_required: self.is_ivc_captcha_required.ok_or_else(|| BuildError::missing_field("is_ivc_captcha_required"))?,
            safety_status: self.safety_status,
            warning_status: self.warning_status,
            on_watchlist: self.on_watchlist.ok_or_else(|| BuildError::missing_field("on_watchlist"))?,
        })
    }
}
