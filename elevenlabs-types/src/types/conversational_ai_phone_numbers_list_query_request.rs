pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiPhoneNumbersListQueryRequest {
    /// Filter by telephony provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<TelephonyProvider>,
    /// Filter by assigned agent ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter by assigned branch ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
}

impl ConversationalAiPhoneNumbersListQueryRequest {
    pub fn builder() -> ConversationalAiPhoneNumbersListQueryRequestBuilder {
        <ConversationalAiPhoneNumbersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiPhoneNumbersListQueryRequestBuilder {
    provider: Option<TelephonyProvider>,
    agent_id: Option<String>,
    branch_id: Option<String>,
}

impl ConversationalAiPhoneNumbersListQueryRequestBuilder {
    pub fn provider(mut self, value: TelephonyProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiPhoneNumbersListQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiPhoneNumbersListQueryRequest, BuildError> {
        Ok(ConversationalAiPhoneNumbersListQueryRequest {
            provider: self.provider,
            agent_id: self.agent_id,
            branch_id: self.branch_id,
        })
    }
}

