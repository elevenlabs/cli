pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReassignPhoneRequest {
    /// The receptionist that answers calls to the number from now on.
    #[serde(default)]
    pub customer_facing_agent_id: String,
}

impl ReassignPhoneRequest {
    pub fn builder() -> ReassignPhoneRequestBuilder {
        <ReassignPhoneRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReassignPhoneRequestBuilder {
    customer_facing_agent_id: Option<String>,
}

impl ReassignPhoneRequestBuilder {
    pub fn customer_facing_agent_id(mut self, value: impl Into<String>) -> Self {
        self.customer_facing_agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReassignPhoneRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_facing_agent_id`](ReassignPhoneRequestBuilder::customer_facing_agent_id)
    pub fn build(self) -> Result<ReassignPhoneRequest, BuildError> {
        Ok(ReassignPhoneRequest {
            customer_facing_agent_id: self.customer_facing_agent_id.ok_or_else(|| BuildError::missing_field("customer_facing_agent_id"))?,
        })
    }
}

