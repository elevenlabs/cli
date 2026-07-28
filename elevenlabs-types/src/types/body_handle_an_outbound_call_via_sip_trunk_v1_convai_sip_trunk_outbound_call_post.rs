pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPost {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub agent_phone_number_id: String,
    #[serde(default)]
    pub to_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephony_call_config: Option<TelephonyCallConfig>,
}

impl BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPost {
    pub fn builder() -> BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPostBuilder {
        <BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPostBuilder {
    agent_id: Option<String>,
    agent_phone_number_id: Option<String>,
    to_number: Option<String>,
    conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
    telephony_call_config: Option<TelephonyCallConfig>,
}

impl BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPostBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.agent_phone_number_id = Some(value.into());
        self
    }

    pub fn to_number(mut self, value: impl Into<String>) -> Self {
        self.to_number = Some(value.into());
        self
    }

    pub fn conversation_initiation_client_data(mut self, value: ConversationInitiationClientDataRequestInput) -> Self {
        self.conversation_initiation_client_data = Some(value);
        self
    }

    pub fn telephony_call_config(mut self, value: TelephonyCallConfig) -> Self {
        self.telephony_call_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPostBuilder::agent_id)
    /// - [`agent_phone_number_id`](BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPostBuilder::agent_phone_number_id)
    /// - [`to_number`](BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPostBuilder::to_number)
    pub fn build(self) -> Result<BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPost, BuildError> {
        Ok(BodyHandleAnOutboundCallViaSipTrunkV1ConvaiSipTrunkOutboundCallPost {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_phone_number_id: self.agent_phone_number_id.ok_or_else(|| BuildError::missing_field("agent_phone_number_id"))?,
            to_number: self.to_number.ok_or_else(|| BuildError::missing_field("to_number"))?,
            conversation_initiation_client_data: self.conversation_initiation_client_data,
            telephony_call_config: self.telephony_call_config,
        })
    }
}

