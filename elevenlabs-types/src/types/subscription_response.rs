pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubscriptionResponse {
    /// The tier of the user's subscription.
    #[serde(default)]
    pub tier: String,
    /// The number of characters used by the user.
    #[serde(default)]
    pub character_count: i64,
    /// The maximum number of characters allowed in the current billing period.
    #[serde(default)]
    pub character_limit: i64,
    /// Deprecated: use `max_credit_limit_extension`. Maximum number of characters that the character limit can be exceeded by. Managed by the workspace admin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_character_limit_extension: Option<i64>,
    /// Maximum number of credits that the credit limit can be exceeded by. Managed by the workspace admin. `"unlimited"` means no cap, `0` means usage-based billing is disabled.
    pub max_credit_limit_extension: SubscriptionResponseMaxCreditLimitExtension,
    /// Whether the workspace is entitled to enter overages (usage-based billing).
    #[serde(default)]
    pub can_extend_character_limit: bool,
    /// Deprecated: use `max_credit_limit_extension != 0`. Whether the user is allowed to extend their character limit.
    #[serde(default)]
    pub allowed_to_extend_character_limit: bool,
    /// The Unix timestamp of the next character count reset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_character_count_reset_unix: Option<i64>,
    /// The number of voice slots used by the user.
    #[serde(default)]
    pub voice_slots_used: i64,
    /// The number of professional voice slots used. For consolidated billing this is the group-wide count across all workspaces in the group; see professional_voice_slots_used_in_workspace for the current workspace only.
    #[serde(default)]
    pub professional_voice_slots_used: i64,
    /// The number of professional voice slots used in the current workspace. For consolidated billing, professional_voice_slots_used counts across all workspaces in the group, while this counts only the current workspace.
    #[serde(default)]
    pub professional_voice_slots_used_in_workspace: i64,
    /// The maximum number of voice slots allowed for the user.
    #[serde(default)]
    pub voice_limit: i64,
    /// The maximum number of voice add/edits allowed for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_voice_add_edits: Option<i64>,
    /// The number of voice add/edits used by the user.
    #[serde(default)]
    pub voice_add_edit_counter: i64,
    /// The maximum number of professional voices allowed for the user.
    #[serde(default)]
    pub professional_voice_limit: i64,
    /// Whether the user can extend their voice limit.
    #[serde(default)]
    pub can_extend_voice_limit: bool,
    /// Whether the user can use instant voice cloning.
    #[serde(default)]
    pub can_use_instant_voice_cloning: bool,
    /// Whether the user can use professional voice cloning.
    #[serde(default)]
    pub can_use_professional_voice_cloning: bool,
    /// The currency of the user's subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// The current usage-based overage cost.
    pub current_overage: Price,
    /// The status of the user's subscription.
    pub status: SubscriptionStatusType,
    /// The billing period of the user's subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<BillingPeriod>,
    /// The character refresh period of the user's subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_refresh_period: Option<CharacterRefreshPeriod>,
}

impl SubscriptionResponse {
    pub fn builder() -> SubscriptionResponseBuilder {
        <SubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionResponseBuilder {
    tier: Option<String>,
    character_count: Option<i64>,
    character_limit: Option<i64>,
    max_character_limit_extension: Option<i64>,
    max_credit_limit_extension: Option<SubscriptionResponseMaxCreditLimitExtension>,
    can_extend_character_limit: Option<bool>,
    allowed_to_extend_character_limit: Option<bool>,
    next_character_count_reset_unix: Option<i64>,
    voice_slots_used: Option<i64>,
    professional_voice_slots_used: Option<i64>,
    professional_voice_slots_used_in_workspace: Option<i64>,
    voice_limit: Option<i64>,
    max_voice_add_edits: Option<i64>,
    voice_add_edit_counter: Option<i64>,
    professional_voice_limit: Option<i64>,
    can_extend_voice_limit: Option<bool>,
    can_use_instant_voice_cloning: Option<bool>,
    can_use_professional_voice_cloning: Option<bool>,
    currency: Option<Currency>,
    current_overage: Option<Price>,
    status: Option<SubscriptionStatusType>,
    billing_period: Option<BillingPeriod>,
    character_refresh_period: Option<CharacterRefreshPeriod>,
}

impl SubscriptionResponseBuilder {
    pub fn tier(mut self, value: impl Into<String>) -> Self {
        self.tier = Some(value.into());
        self
    }

    pub fn character_count(mut self, value: i64) -> Self {
        self.character_count = Some(value);
        self
    }

    pub fn character_limit(mut self, value: i64) -> Self {
        self.character_limit = Some(value);
        self
    }

    pub fn max_character_limit_extension(mut self, value: i64) -> Self {
        self.max_character_limit_extension = Some(value);
        self
    }

    pub fn max_credit_limit_extension(mut self, value: SubscriptionResponseMaxCreditLimitExtension) -> Self {
        self.max_credit_limit_extension = Some(value);
        self
    }

    pub fn can_extend_character_limit(mut self, value: bool) -> Self {
        self.can_extend_character_limit = Some(value);
        self
    }

    pub fn allowed_to_extend_character_limit(mut self, value: bool) -> Self {
        self.allowed_to_extend_character_limit = Some(value);
        self
    }

    pub fn next_character_count_reset_unix(mut self, value: i64) -> Self {
        self.next_character_count_reset_unix = Some(value);
        self
    }

    pub fn voice_slots_used(mut self, value: i64) -> Self {
        self.voice_slots_used = Some(value);
        self
    }

    pub fn professional_voice_slots_used(mut self, value: i64) -> Self {
        self.professional_voice_slots_used = Some(value);
        self
    }

    pub fn professional_voice_slots_used_in_workspace(mut self, value: i64) -> Self {
        self.professional_voice_slots_used_in_workspace = Some(value);
        self
    }

    pub fn voice_limit(mut self, value: i64) -> Self {
        self.voice_limit = Some(value);
        self
    }

    pub fn max_voice_add_edits(mut self, value: i64) -> Self {
        self.max_voice_add_edits = Some(value);
        self
    }

    pub fn voice_add_edit_counter(mut self, value: i64) -> Self {
        self.voice_add_edit_counter = Some(value);
        self
    }

    pub fn professional_voice_limit(mut self, value: i64) -> Self {
        self.professional_voice_limit = Some(value);
        self
    }

    pub fn can_extend_voice_limit(mut self, value: bool) -> Self {
        self.can_extend_voice_limit = Some(value);
        self
    }

    pub fn can_use_instant_voice_cloning(mut self, value: bool) -> Self {
        self.can_use_instant_voice_cloning = Some(value);
        self
    }

    pub fn can_use_professional_voice_cloning(mut self, value: bool) -> Self {
        self.can_use_professional_voice_cloning = Some(value);
        self
    }

    pub fn currency(mut self, value: Currency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn current_overage(mut self, value: Price) -> Self {
        self.current_overage = Some(value);
        self
    }

    pub fn status(mut self, value: SubscriptionStatusType) -> Self {
        self.status = Some(value);
        self
    }

    pub fn billing_period(mut self, value: BillingPeriod) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn character_refresh_period(mut self, value: CharacterRefreshPeriod) -> Self {
        self.character_refresh_period = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tier`](SubscriptionResponseBuilder::tier)
    /// - [`character_count`](SubscriptionResponseBuilder::character_count)
    /// - [`character_limit`](SubscriptionResponseBuilder::character_limit)
    /// - [`max_credit_limit_extension`](SubscriptionResponseBuilder::max_credit_limit_extension)
    /// - [`can_extend_character_limit`](SubscriptionResponseBuilder::can_extend_character_limit)
    /// - [`allowed_to_extend_character_limit`](SubscriptionResponseBuilder::allowed_to_extend_character_limit)
    /// - [`voice_slots_used`](SubscriptionResponseBuilder::voice_slots_used)
    /// - [`professional_voice_slots_used`](SubscriptionResponseBuilder::professional_voice_slots_used)
    /// - [`professional_voice_slots_used_in_workspace`](SubscriptionResponseBuilder::professional_voice_slots_used_in_workspace)
    /// - [`voice_limit`](SubscriptionResponseBuilder::voice_limit)
    /// - [`voice_add_edit_counter`](SubscriptionResponseBuilder::voice_add_edit_counter)
    /// - [`professional_voice_limit`](SubscriptionResponseBuilder::professional_voice_limit)
    /// - [`can_extend_voice_limit`](SubscriptionResponseBuilder::can_extend_voice_limit)
    /// - [`can_use_instant_voice_cloning`](SubscriptionResponseBuilder::can_use_instant_voice_cloning)
    /// - [`can_use_professional_voice_cloning`](SubscriptionResponseBuilder::can_use_professional_voice_cloning)
    /// - [`current_overage`](SubscriptionResponseBuilder::current_overage)
    /// - [`status`](SubscriptionResponseBuilder::status)
    pub fn build(self) -> Result<SubscriptionResponse, BuildError> {
        Ok(SubscriptionResponse {
            tier: self.tier.ok_or_else(|| BuildError::missing_field("tier"))?,
            character_count: self.character_count.ok_or_else(|| BuildError::missing_field("character_count"))?,
            character_limit: self.character_limit.ok_or_else(|| BuildError::missing_field("character_limit"))?,
            max_character_limit_extension: self.max_character_limit_extension,
            max_credit_limit_extension: self.max_credit_limit_extension.ok_or_else(|| BuildError::missing_field("max_credit_limit_extension"))?,
            can_extend_character_limit: self.can_extend_character_limit.ok_or_else(|| BuildError::missing_field("can_extend_character_limit"))?,
            allowed_to_extend_character_limit: self.allowed_to_extend_character_limit.ok_or_else(|| BuildError::missing_field("allowed_to_extend_character_limit"))?,
            next_character_count_reset_unix: self.next_character_count_reset_unix,
            voice_slots_used: self.voice_slots_used.ok_or_else(|| BuildError::missing_field("voice_slots_used"))?,
            professional_voice_slots_used: self.professional_voice_slots_used.ok_or_else(|| BuildError::missing_field("professional_voice_slots_used"))?,
            professional_voice_slots_used_in_workspace: self.professional_voice_slots_used_in_workspace.ok_or_else(|| BuildError::missing_field("professional_voice_slots_used_in_workspace"))?,
            voice_limit: self.voice_limit.ok_or_else(|| BuildError::missing_field("voice_limit"))?,
            max_voice_add_edits: self.max_voice_add_edits,
            voice_add_edit_counter: self.voice_add_edit_counter.ok_or_else(|| BuildError::missing_field("voice_add_edit_counter"))?,
            professional_voice_limit: self.professional_voice_limit.ok_or_else(|| BuildError::missing_field("professional_voice_limit"))?,
            can_extend_voice_limit: self.can_extend_voice_limit.ok_or_else(|| BuildError::missing_field("can_extend_voice_limit"))?,
            can_use_instant_voice_cloning: self.can_use_instant_voice_cloning.ok_or_else(|| BuildError::missing_field("can_use_instant_voice_cloning"))?,
            can_use_professional_voice_cloning: self.can_use_professional_voice_cloning.ok_or_else(|| BuildError::missing_field("can_use_professional_voice_cloning"))?,
            currency: self.currency,
            current_overage: self.current_overage.ok_or_else(|| BuildError::missing_field("current_overage"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            billing_period: self.billing_period,
            character_refresh_period: self.character_refresh_period,
        })
    }
}
