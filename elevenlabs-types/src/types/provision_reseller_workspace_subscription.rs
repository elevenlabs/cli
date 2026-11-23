pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProvisionResellerWorkspaceSubscription {
    #[serde(default)]
    pub tier_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<ProvisionResellerWorkspaceSubscriptionBillingPeriod>,
}

impl ProvisionResellerWorkspaceSubscription {
    pub fn builder() -> ProvisionResellerWorkspaceSubscriptionBuilder {
        <ProvisionResellerWorkspaceSubscriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProvisionResellerWorkspaceSubscriptionBuilder {
    tier_id: Option<String>,
    billing_period: Option<ProvisionResellerWorkspaceSubscriptionBillingPeriod>,
}

impl ProvisionResellerWorkspaceSubscriptionBuilder {
    pub fn tier_id(mut self, value: impl Into<String>) -> Self {
        self.tier_id = Some(value.into());
        self
    }

    pub fn billing_period(mut self, value: ProvisionResellerWorkspaceSubscriptionBillingPeriod) -> Self {
        self.billing_period = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProvisionResellerWorkspaceSubscription`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tier_id`](ProvisionResellerWorkspaceSubscriptionBuilder::tier_id)
    pub fn build(self) -> Result<ProvisionResellerWorkspaceSubscription, BuildError> {
        Ok(ProvisionResellerWorkspaceSubscription {
            tier_id: self.tier_id.ok_or_else(|| BuildError::missing_field("tier_id"))?,
            billing_period: self.billing_period,
        })
    }
}
