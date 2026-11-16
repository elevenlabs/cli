pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResellerWorkspaceOnboardingResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_completed_at_unix: Option<i64>,
    /// The receptionist onboarding created or promoted.
    #[serde(default)]
    pub customer_facing_agent_id: String,
}

impl ResellerWorkspaceOnboardingResponse {
    pub fn builder() -> ResellerWorkspaceOnboardingResponseBuilder {
        <ResellerWorkspaceOnboardingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResellerWorkspaceOnboardingResponseBuilder {
    onboarding_completed_at_unix: Option<i64>,
    customer_facing_agent_id: Option<String>,
}

impl ResellerWorkspaceOnboardingResponseBuilder {
    pub fn onboarding_completed_at_unix(mut self, value: i64) -> Self {
        self.onboarding_completed_at_unix = Some(value);
        self
    }

    pub fn customer_facing_agent_id(mut self, value: impl Into<String>) -> Self {
        self.customer_facing_agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResellerWorkspaceOnboardingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_facing_agent_id`](ResellerWorkspaceOnboardingResponseBuilder::customer_facing_agent_id)
    pub fn build(self) -> Result<ResellerWorkspaceOnboardingResponse, BuildError> {
        Ok(ResellerWorkspaceOnboardingResponse {
            onboarding_completed_at_unix: self.onboarding_completed_at_unix,
            customer_facing_agent_id: self.customer_facing_agent_id.ok_or_else(|| BuildError::missing_field("customer_facing_agent_id"))?,
        })
    }
}
