pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionProvisionResellerWorkspaceRequest {
    #[serde(default)]
    pub business_name: String,
    /// The business's vertical, used with `complete_onboarding`. Without it, onboarding picks the vertical as usual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical: Option<ReceptionVertical>,
    /// Bill the workspace's own customer instead of the reseller: it checks out and sees its billing itself, and the reseller earns a commission. Only for a reseller on standard billing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_customer: Option<bool>,
    /// Paid tier to subscribe to. Omitted leaves the workspace with no subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<ProvisionResellerWorkspaceSubscription>,
    /// A free number from the reseller's phone pool, reserved for the workspace until onboarding binds it to the receptionist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// People to invite once the workspace exists, capped by the tier's team size. Best-effort: each outcome is in `member_results`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TeamInviteRequest>>,
    /// Finish setup right away, as `POST /v1/reception/reseller/workspaces/{workspace_id}/complete-onboarding` does. Best-effort: `onboarding_completed_at_unix` stays null on failure, and that call can be retried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_onboarding: Option<bool>,
    /// Client-generated key that makes a retry safe. The same key and body within 24 hours returns the first result without running again. While that first request is still running, a retry gets 409 `idempotency_key_in_progress`; a different body with the same key gets 400 `idempotency_key_reused`. A request that fails is not saved, so it can be retried with the same key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

impl ReceptionProvisionResellerWorkspaceRequest {
    pub fn builder() -> ReceptionProvisionResellerWorkspaceRequestBuilder {
        <ReceptionProvisionResellerWorkspaceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionProvisionResellerWorkspaceRequestBuilder {
    business_name: Option<String>,
    vertical: Option<ReceptionVertical>,
    charge_customer: Option<bool>,
    subscription: Option<ProvisionResellerWorkspaceSubscription>,
    phone_number: Option<String>,
    members: Option<Vec<TeamInviteRequest>>,
    complete_onboarding: Option<bool>,
    idempotency_key: Option<String>,
}

impl ReceptionProvisionResellerWorkspaceRequestBuilder {
    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn vertical(mut self, value: ReceptionVertical) -> Self {
        self.vertical = Some(value);
        self
    }

    pub fn charge_customer(mut self, value: bool) -> Self {
        self.charge_customer = Some(value);
        self
    }

    pub fn subscription(mut self, value: ProvisionResellerWorkspaceSubscription) -> Self {
        self.subscription = Some(value);
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn members(mut self, value: Vec<TeamInviteRequest>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn complete_onboarding(mut self, value: bool) -> Self {
        self.complete_onboarding = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceptionProvisionResellerWorkspaceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_name`](ReceptionProvisionResellerWorkspaceRequestBuilder::business_name)
    pub fn build(self) -> Result<ReceptionProvisionResellerWorkspaceRequest, BuildError> {
        Ok(ReceptionProvisionResellerWorkspaceRequest {
            business_name: self.business_name.ok_or_else(|| BuildError::missing_field("business_name"))?,
            vertical: self.vertical,
            charge_customer: self.charge_customer,
            subscription: self.subscription,
            phone_number: self.phone_number,
            members: self.members,
            complete_onboarding: self.complete_onboarding,
            idempotency_key: self.idempotency_key,
        })
    }
}

