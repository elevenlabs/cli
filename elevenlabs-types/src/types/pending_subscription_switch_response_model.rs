pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PendingSubscriptionSwitchResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The tier to change to.
    pub next_tier: PendingSubscriptionSwitchResponseModelNextTier,
    /// The billing period to change to.
    pub next_billing_period: BillingPeriod,
    /// The timestamp of the change.
    #[serde(default)]
    pub timestamp_seconds: i64,
}

impl PendingSubscriptionSwitchResponseModel {
    pub fn builder() -> PendingSubscriptionSwitchResponseModelBuilder {
        <PendingSubscriptionSwitchResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PendingSubscriptionSwitchResponseModelBuilder {
    kind: Option<String>,
    next_tier: Option<PendingSubscriptionSwitchResponseModelNextTier>,
    next_billing_period: Option<BillingPeriod>,
    timestamp_seconds: Option<i64>,
}

impl PendingSubscriptionSwitchResponseModelBuilder {
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn next_tier(mut self, value: PendingSubscriptionSwitchResponseModelNextTier) -> Self {
        self.next_tier = Some(value);
        self
    }

    pub fn next_billing_period(mut self, value: BillingPeriod) -> Self {
        self.next_billing_period = Some(value);
        self
    }

    pub fn timestamp_seconds(mut self, value: i64) -> Self {
        self.timestamp_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PendingSubscriptionSwitchResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`next_tier`](PendingSubscriptionSwitchResponseModelBuilder::next_tier)
    /// - [`next_billing_period`](PendingSubscriptionSwitchResponseModelBuilder::next_billing_period)
    /// - [`timestamp_seconds`](PendingSubscriptionSwitchResponseModelBuilder::timestamp_seconds)
    pub fn build(self) -> Result<PendingSubscriptionSwitchResponseModel, BuildError> {
        Ok(PendingSubscriptionSwitchResponseModel {
            kind: self.kind,
            next_tier: self.next_tier.ok_or_else(|| BuildError::missing_field("next_tier"))?,
            next_billing_period: self.next_billing_period.ok_or_else(|| BuildError::missing_field("next_billing_period"))?,
            timestamp_seconds: self.timestamp_seconds.ok_or_else(|| BuildError::missing_field("timestamp_seconds"))?,
        })
    }
}
