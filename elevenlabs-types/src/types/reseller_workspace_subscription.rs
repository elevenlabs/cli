pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A reseller-managed paid subscription, as the reseller can act on it.
/// 
/// Present only for a workspace the reseller actually sells to: untiered and
/// trial workspaces have no subscription to change or end, and are assigned to
/// instead.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResellerWorkspaceSubscription {
    #[serde(default)]
    pub tier_id: String,
    /// Cadence being billed. None only for a subscription mid-provisioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<ResellerWorkspaceSubscriptionBillingPeriod>,
    /// When the current period ends, and so when a deferred change lands.
    #[serde(default)]
    pub billing_period_end_unix: i64,
    /// When the subscription ends. Set means a pending unassignment the reseller can still undo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_unix: Option<i64>,
    /// Tier this workspace drops to at the period boundary, if a downgrade is already scheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_downgrade_tier: Option<String>,
    /// When that scheduled downgrade takes effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_downgrade_at_unix: Option<i64>,
}

impl ResellerWorkspaceSubscription {
    pub fn builder() -> ResellerWorkspaceSubscriptionBuilder {
        <ResellerWorkspaceSubscriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResellerWorkspaceSubscriptionBuilder {
    tier_id: Option<String>,
    billing_period: Option<ResellerWorkspaceSubscriptionBillingPeriod>,
    billing_period_end_unix: Option<i64>,
    cancel_at_unix: Option<i64>,
    pending_downgrade_tier: Option<String>,
    pending_downgrade_at_unix: Option<i64>,
}

impl ResellerWorkspaceSubscriptionBuilder {
    pub fn tier_id(mut self, value: impl Into<String>) -> Self {
        self.tier_id = Some(value.into());
        self
    }

    pub fn billing_period(mut self, value: ResellerWorkspaceSubscriptionBillingPeriod) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn billing_period_end_unix(mut self, value: i64) -> Self {
        self.billing_period_end_unix = Some(value);
        self
    }

    pub fn cancel_at_unix(mut self, value: i64) -> Self {
        self.cancel_at_unix = Some(value);
        self
    }

    pub fn pending_downgrade_tier(mut self, value: impl Into<String>) -> Self {
        self.pending_downgrade_tier = Some(value.into());
        self
    }

    pub fn pending_downgrade_at_unix(mut self, value: i64) -> Self {
        self.pending_downgrade_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResellerWorkspaceSubscription`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tier_id`](ResellerWorkspaceSubscriptionBuilder::tier_id)
    /// - [`billing_period_end_unix`](ResellerWorkspaceSubscriptionBuilder::billing_period_end_unix)
    pub fn build(self) -> Result<ResellerWorkspaceSubscription, BuildError> {
        Ok(ResellerWorkspaceSubscription {
            tier_id: self.tier_id.ok_or_else(|| BuildError::missing_field("tier_id"))?,
            billing_period: self.billing_period,
            billing_period_end_unix: self.billing_period_end_unix.ok_or_else(|| BuildError::missing_field("billing_period_end_unix"))?,
            cancel_at_unix: self.cancel_at_unix,
            pending_downgrade_tier: self.pending_downgrade_tier,
            pending_downgrade_at_unix: self.pending_downgrade_at_unix,
        })
    }
}
