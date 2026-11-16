pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChangeResellerTierRequest {
    #[serde(default)]
    pub tier_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<ChangeResellerTierRequestBillingPeriod>,
    /// Client-generated key that makes a retry safe. The same key and body within 24 hours returns the first result without running again. While that first request is still running, a retry gets 409 `idempotency_key_in_progress`; a different body with the same key gets 400 `idempotency_key_reused`. A request that fails is not saved, so it can be retried with the same key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

impl ChangeResellerTierRequest {
    pub fn builder() -> ChangeResellerTierRequestBuilder {
        <ChangeResellerTierRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeResellerTierRequestBuilder {
    tier_id: Option<String>,
    billing_period: Option<ChangeResellerTierRequestBillingPeriod>,
    idempotency_key: Option<String>,
}

impl ChangeResellerTierRequestBuilder {
    pub fn tier_id(mut self, value: impl Into<String>) -> Self {
        self.tier_id = Some(value.into());
        self
    }

    pub fn billing_period(mut self, value: ChangeResellerTierRequestBillingPeriod) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChangeResellerTierRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tier_id`](ChangeResellerTierRequestBuilder::tier_id)
    pub fn build(self) -> Result<ChangeResellerTierRequest, BuildError> {
        Ok(ChangeResellerTierRequest {
            tier_id: self.tier_id.ok_or_else(|| BuildError::missing_field("tier_id"))?,
            billing_period: self.billing_period,
            idempotency_key: self.idempotency_key,
        })
    }
}

