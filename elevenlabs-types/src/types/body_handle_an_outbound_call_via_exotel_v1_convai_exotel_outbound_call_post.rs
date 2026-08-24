pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPost {
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

impl BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPost {
    pub fn builder() -> BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPostBuilder {
        <BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPostBuilder {
    agent_id: Option<String>,
    agent_phone_number_id: Option<String>,
    to_number: Option<String>,
    conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
    telephony_call_config: Option<TelephonyCallConfig>,
}

impl BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPostBuilder {
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

    /// Consumes the builder and constructs a [`BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPostBuilder::agent_id)
    /// - [`agent_phone_number_id`](BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPostBuilder::agent_phone_number_id)
    /// - [`to_number`](BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPostBuilder::to_number)
    pub fn build(self) -> Result<BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPost, BuildError> {
        Ok(BodyHandleAnOutboundCallViaExotelV1ConvaiExotelOutboundCallPost {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_phone_number_id: self.agent_phone_number_id.ok_or_else(|| BuildError::missing_field("agent_phone_number_id"))?,
            to_number: self.to_number.ok_or_else(|| BuildError::missing_field("to_number"))?,
            conversation_initiation_client_data: self.conversation_initiation_client_data,
            telephony_call_config: self.telephony_call_config,
        })
    }
}

