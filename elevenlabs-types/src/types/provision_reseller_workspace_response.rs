pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProvisionResellerWorkspaceResponse {
    #[serde(default)]
    pub workspace_id: String,
    #[serde(default)]
    pub business_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_tier: Option<String>,
    #[serde(default)]
    pub receptionist_count: i64,
    #[serde(rename = "calls_last_30_days")]
    #[serde(default)]
    pub calls_last30days: i64,
    #[serde(default)]
    pub created_at_unix: i64,
    /// The reseller-managed subscription, when this workspace has one. None means the reseller sells nothing here: no tier is assigned yet, or the workspace pays for its own subscription (see charge_customer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<ResellerWorkspaceSubscription>,
    /// The workspace pays for its own subscription, so its tier can't be assigned or changed here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_customer: Option<bool>,
    /// A charge for this workspace is unpaid. Assigning or changing a tier is refused until it clears, so surface it rather than letting it fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_payment_issue: Option<bool>,
    /// When onboarding finished. Until then the onboarding wizard may replace or remove the workspace's business data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_completed_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_results: Option<Vec<TeamInviteResult>>,
}

impl ProvisionResellerWorkspaceResponse {
    pub fn builder() -> ProvisionResellerWorkspaceResponseBuilder {
        <ProvisionResellerWorkspaceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProvisionResellerWorkspaceResponseBuilder {
    workspace_id: Option<String>,
    business_name: Option<String>,
    billing_tier: Option<String>,
    receptionist_count: Option<i64>,
    calls_last30days: Option<i64>,
    created_at_unix: Option<i64>,
    subscription: Option<ResellerWorkspaceSubscription>,
    charge_customer: Option<bool>,
    has_payment_issue: Option<bool>,
    onboarding_completed_at_unix: Option<i64>,
    member_results: Option<Vec<TeamInviteResult>>,
}

impl ProvisionResellerWorkspaceResponseBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn billing_tier(mut self, value: impl Into<String>) -> Self {
        self.billing_tier = Some(value.into());
        self
    }

    pub fn receptionist_count(mut self, value: i64) -> Self {
        self.receptionist_count = Some(value);
        self
    }

    pub fn calls_last30days(mut self, value: i64) -> Self {
        self.calls_last30days = Some(value);
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn subscription(mut self, value: ResellerWorkspaceSubscription) -> Self {
        self.subscription = Some(value);
        self
    }

    pub fn charge_customer(mut self, value: bool) -> Self {
        self.charge_customer = Some(value);
        self
    }

    pub fn has_payment_issue(mut self, value: bool) -> Self {
        self.has_payment_issue = Some(value);
        self
    }

    pub fn onboarding_completed_at_unix(mut self, value: i64) -> Self {
        self.onboarding_completed_at_unix = Some(value);
        self
    }

    pub fn member_results(mut self, value: Vec<TeamInviteResult>) -> Self {
        self.member_results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProvisionResellerWorkspaceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_id`](ProvisionResellerWorkspaceResponseBuilder::workspace_id)
    /// - [`business_name`](ProvisionResellerWorkspaceResponseBuilder::business_name)
    /// - [`receptionist_count`](ProvisionResellerWorkspaceResponseBuilder::receptionist_count)
    /// - [`calls_last30days`](ProvisionResellerWorkspaceResponseBuilder::calls_last30days)
    /// - [`created_at_unix`](ProvisionResellerWorkspaceResponseBuilder::created_at_unix)
    pub fn build(self) -> Result<ProvisionResellerWorkspaceResponse, BuildError> {
        Ok(ProvisionResellerWorkspaceResponse {
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            business_name: self.business_name.ok_or_else(|| BuildError::missing_field("business_name"))?,
            billing_tier: self.billing_tier,
            receptionist_count: self.receptionist_count.ok_or_else(|| BuildError::missing_field("receptionist_count"))?,
            calls_last30days: self.calls_last30days.ok_or_else(|| BuildError::missing_field("calls_last30days"))?,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            subscription: self.subscription,
            charge_customer: self.charge_customer,
            has_payment_issue: self.has_payment_issue,
            onboarding_completed_at_unix: self.onboarding_completed_at_unix,
            member_results: self.member_results,
        })
    }
}
