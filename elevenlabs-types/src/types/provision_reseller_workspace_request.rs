pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProvisionResellerWorkspaceRequest {
    #[serde(default)]
    pub business_name: String,
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

impl ProvisionResellerWorkspaceRequest {
    pub fn builder() -> ProvisionResellerWorkspaceRequestBuilder {
        <ProvisionResellerWorkspaceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProvisionResellerWorkspaceRequestBuilder {
    business_name: Option<String>,
    subscription: Option<ProvisionResellerWorkspaceSubscription>,
    phone_number: Option<String>,
    members: Option<Vec<TeamInviteRequest>>,
    complete_onboarding: Option<bool>,
    idempotency_key: Option<String>,
}

impl ProvisionResellerWorkspaceRequestBuilder {
    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
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

    /// Consumes the builder and constructs a [`ProvisionResellerWorkspaceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_name`](ProvisionResellerWorkspaceRequestBuilder::business_name)
    pub fn build(self) -> Result<ProvisionResellerWorkspaceRequest, BuildError> {
        Ok(ProvisionResellerWorkspaceRequest {
            business_name: self.business_name.ok_or_else(|| BuildError::missing_field("business_name"))?,
            subscription: self.subscription,
            phone_number: self.phone_number,
            members: self.members,
            complete_onboarding: self.complete_onboarding,
            idempotency_key: self.idempotency_key,
        })
    }
}

