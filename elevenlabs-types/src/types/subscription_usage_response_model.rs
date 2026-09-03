pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionUsageResponseModel {
    /// The rollover credits quota.
    #[serde(default)]
    pub rollover_credits_quota: i64,
    /// The subscription cycle credits quota.
    #[serde(default)]
    pub subscription_cycle_credits_quota: i64,
    /// The manually gifted credits quota.
    #[serde(default)]
    pub manually_gifted_credits_quota: i64,
    /// The payg credits quota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payg_credits_quota: Option<i64>,
    /// The rollover credits used.
    #[serde(default)]
    pub rollover_credits_used: i64,
    /// The subscription cycle credits used.
    #[serde(default)]
    pub subscription_cycle_credits_used: i64,
    /// The manually gifted credits used.
    #[serde(default)]
    pub manually_gifted_credits_used: i64,
    /// The payg credits used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payg_credits_used: Option<i64>,
    /// The paid usage based credits used.
    #[serde(default)]
    pub paid_usage_based_credits_used: i64,
    /// The actual reported credits.
    #[serde(default)]
    pub actual_reported_credits: i64,
}

impl SubscriptionUsageResponseModel {
    pub fn builder() -> SubscriptionUsageResponseModelBuilder {
        <SubscriptionUsageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionUsageResponseModelBuilder {
    rollover_credits_quota: Option<i64>,
    subscription_cycle_credits_quota: Option<i64>,
    manually_gifted_credits_quota: Option<i64>,
    payg_credits_quota: Option<i64>,
    rollover_credits_used: Option<i64>,
    subscription_cycle_credits_used: Option<i64>,
    manually_gifted_credits_used: Option<i64>,
    payg_credits_used: Option<i64>,
    paid_usage_based_credits_used: Option<i64>,
    actual_reported_credits: Option<i64>,
}

impl SubscriptionUsageResponseModelBuilder {
    pub fn rollover_credits_quota(mut self, value: i64) -> Self {
        self.rollover_credits_quota = Some(value);
        self
    }

    pub fn subscription_cycle_credits_quota(mut self, value: i64) -> Self {
        self.subscription_cycle_credits_quota = Some(value);
        self
    }

    pub fn manually_gifted_credits_quota(mut self, value: i64) -> Self {
        self.manually_gifted_credits_quota = Some(value);
        self
    }

    pub fn payg_credits_quota(mut self, value: i64) -> Self {
        self.payg_credits_quota = Some(value);
        self
    }

    pub fn rollover_credits_used(mut self, value: i64) -> Self {
        self.rollover_credits_used = Some(value);
        self
    }

    pub fn subscription_cycle_credits_used(mut self, value: i64) -> Self {
        self.subscription_cycle_credits_used = Some(value);
        self
    }

    pub fn manually_gifted_credits_used(mut self, value: i64) -> Self {
        self.manually_gifted_credits_used = Some(value);
        self
    }

    pub fn payg_credits_used(mut self, value: i64) -> Self {
        self.payg_credits_used = Some(value);
        self
    }

    pub fn paid_usage_based_credits_used(mut self, value: i64) -> Self {
        self.paid_usage_based_credits_used = Some(value);
        self
    }

    pub fn actual_reported_credits(mut self, value: i64) -> Self {
        self.actual_reported_credits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionUsageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rollover_credits_quota`](SubscriptionUsageResponseModelBuilder::rollover_credits_quota)
    /// - [`subscription_cycle_credits_quota`](SubscriptionUsageResponseModelBuilder::subscription_cycle_credits_quota)
    /// - [`manually_gifted_credits_quota`](SubscriptionUsageResponseModelBuilder::manually_gifted_credits_quota)
    /// - [`rollover_credits_used`](SubscriptionUsageResponseModelBuilder::rollover_credits_used)
    /// - [`subscription_cycle_credits_used`](SubscriptionUsageResponseModelBuilder::subscription_cycle_credits_used)
    /// - [`manually_gifted_credits_used`](SubscriptionUsageResponseModelBuilder::manually_gifted_credits_used)
    /// - [`paid_usage_based_credits_used`](SubscriptionUsageResponseModelBuilder::paid_usage_based_credits_used)
    /// - [`actual_reported_credits`](SubscriptionUsageResponseModelBuilder::actual_reported_credits)
    pub fn build(self) -> Result<SubscriptionUsageResponseModel, BuildError> {
        Ok(SubscriptionUsageResponseModel {
            rollover_credits_quota: self.rollover_credits_quota.ok_or_else(|| BuildError::missing_field("rollover_credits_quota"))?,
            subscription_cycle_credits_quota: self.subscription_cycle_credits_quota.ok_or_else(|| BuildError::missing_field("subscription_cycle_credits_quota"))?,
            manually_gifted_credits_quota: self.manually_gifted_credits_quota.ok_or_else(|| BuildError::missing_field("manually_gifted_credits_quota"))?,
            payg_credits_quota: self.payg_credits_quota,
            rollover_credits_used: self.rollover_credits_used.ok_or_else(|| BuildError::missing_field("rollover_credits_used"))?,
            subscription_cycle_credits_used: self.subscription_cycle_credits_used.ok_or_else(|| BuildError::missing_field("subscription_cycle_credits_used"))?,
            manually_gifted_credits_used: self.manually_gifted_credits_used.ok_or_else(|| BuildError::missing_field("manually_gifted_credits_used"))?,
            payg_credits_used: self.payg_credits_used,
            paid_usage_based_credits_used: self.paid_usage_based_credits_used.ok_or_else(|| BuildError::missing_field("paid_usage_based_credits_used"))?,
            actual_reported_credits: self.actual_reported_credits.ok_or_else(|| BuildError::missing_field("actual_reported_credits"))?,
        })
    }
}
