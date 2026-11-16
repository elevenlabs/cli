pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A workspace's next renewal, estimated before its invoice is created.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResellerUpcomingPayment {
    #[serde(default)]
    pub workspace_id: String,
    #[serde(default)]
    pub workspace_name: String,
    #[serde(default)]
    pub subscription: ResellerWorkspaceSubscription,
    /// The plan the renewal charges upfront: the pending downgrade's tier when there is one.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub plan_usd: f64,
    /// This period's usage beyond its included minutes so far, charged at the renewal and still growing.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub overage_so_far_usd: f64,
}

impl ResellerUpcomingPayment {
    pub fn builder() -> ResellerUpcomingPaymentBuilder {
        <ResellerUpcomingPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResellerUpcomingPaymentBuilder {
    workspace_id: Option<String>,
    workspace_name: Option<String>,
    subscription: Option<ResellerWorkspaceSubscription>,
    plan_usd: Option<f64>,
    overage_so_far_usd: Option<f64>,
}

impl ResellerUpcomingPaymentBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn workspace_name(mut self, value: impl Into<String>) -> Self {
        self.workspace_name = Some(value.into());
        self
    }

    pub fn subscription(mut self, value: ResellerWorkspaceSubscription) -> Self {
        self.subscription = Some(value);
        self
    }

    pub fn plan_usd(mut self, value: f64) -> Self {
        self.plan_usd = Some(value);
        self
    }

    pub fn overage_so_far_usd(mut self, value: f64) -> Self {
        self.overage_so_far_usd = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResellerUpcomingPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_id`](ResellerUpcomingPaymentBuilder::workspace_id)
    /// - [`workspace_name`](ResellerUpcomingPaymentBuilder::workspace_name)
    /// - [`subscription`](ResellerUpcomingPaymentBuilder::subscription)
    /// - [`plan_usd`](ResellerUpcomingPaymentBuilder::plan_usd)
    /// - [`overage_so_far_usd`](ResellerUpcomingPaymentBuilder::overage_so_far_usd)
    pub fn build(self) -> Result<ResellerUpcomingPayment, BuildError> {
        Ok(ResellerUpcomingPayment {
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            workspace_name: self.workspace_name.ok_or_else(|| BuildError::missing_field("workspace_name"))?,
            subscription: self.subscription.ok_or_else(|| BuildError::missing_field("subscription"))?,
            plan_usd: self.plan_usd.ok_or_else(|| BuildError::missing_field("plan_usd"))?,
            overage_so_far_usd: self.overage_so_far_usd.ok_or_else(|| BuildError::missing_field("overage_so_far_usd"))?,
        })
    }
}
