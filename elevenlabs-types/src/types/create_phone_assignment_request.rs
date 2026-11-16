pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePhoneAssignmentRequest {
    /// The receptionist that answers calls to the number.
    #[serde(default)]
    pub customer_facing_agent_id: String,
    /// Preferred US or Canadian area codes, e.g. `["415", "628"]`. Codes outside the business's country are ignored, and any number is given when none of these is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_codes: Option<Vec<String>>,
    /// Local or mobile, in a country that requires paperwork before a number takes calls. Leave unset to try mobile first. Ignored in the US and Canada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_type: Option<ApplicationNumberType>,
    /// Client-generated key that makes a retry safe. The same key and body within 24 hours returns the first result without running again. While that first request is still running, a retry gets 409 `idempotency_key_in_progress`; a different body with the same key gets 400 `idempotency_key_reused`. A request that fails is not saved, so it can be retried with the same key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

impl CreatePhoneAssignmentRequest {
    pub fn builder() -> CreatePhoneAssignmentRequestBuilder {
        <CreatePhoneAssignmentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePhoneAssignmentRequestBuilder {
    customer_facing_agent_id: Option<String>,
    area_codes: Option<Vec<String>>,
    number_type: Option<ApplicationNumberType>,
    idempotency_key: Option<String>,
}

impl CreatePhoneAssignmentRequestBuilder {
    pub fn customer_facing_agent_id(mut self, value: impl Into<String>) -> Self {
        self.customer_facing_agent_id = Some(value.into());
        self
    }

    pub fn area_codes(mut self, value: Vec<String>) -> Self {
        self.area_codes = Some(value);
        self
    }

    pub fn number_type(mut self, value: ApplicationNumberType) -> Self {
        self.number_type = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePhoneAssignmentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_facing_agent_id`](CreatePhoneAssignmentRequestBuilder::customer_facing_agent_id)
    pub fn build(self) -> Result<CreatePhoneAssignmentRequest, BuildError> {
        Ok(CreatePhoneAssignmentRequest {
            customer_facing_agent_id: self.customer_facing_agent_id.ok_or_else(|| BuildError::missing_field("customer_facing_agent_id"))?,
            area_codes: self.area_codes,
            number_type: self.number_type,
            idempotency_key: self.idempotency_key,
        })
    }
}

