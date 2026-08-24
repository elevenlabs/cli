pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPost {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub agent_phone_number_id: String,
    #[serde(default)]
    pub to_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
    /// Whether let Twilio record the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_recording_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephony_call_config: Option<TelephonyCallConfig>,
}

impl BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPost {
    pub fn builder() -> BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPostBuilder {
        <BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPostBuilder {
    agent_id: Option<String>,
    agent_phone_number_id: Option<String>,
    to_number: Option<String>,
    conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
    call_recording_enabled: Option<bool>,
    telephony_call_config: Option<TelephonyCallConfig>,
}

impl BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPostBuilder {
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

    pub fn call_recording_enabled(mut self, value: bool) -> Self {
        self.call_recording_enabled = Some(value);
        self
    }

    pub fn telephony_call_config(mut self, value: TelephonyCallConfig) -> Self {
        self.telephony_call_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPostBuilder::agent_id)
    /// - [`agent_phone_number_id`](BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPostBuilder::agent_phone_number_id)
    /// - [`to_number`](BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPostBuilder::to_number)
    pub fn build(self) -> Result<BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPost, BuildError> {
        Ok(BodyHandleAnOutboundCallViaTwilioV1ConvaiTwilioOutboundCallPost {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_phone_number_id: self.agent_phone_number_id.ok_or_else(|| BuildError::missing_field("agent_phone_number_id"))?,
            to_number: self.to_number.ok_or_else(|| BuildError::missing_field("to_number"))?,
            conversation_initiation_client_data: self.conversation_initiation_client_data,
            call_recording_enabled: self.call_recording_enabled,
            telephony_call_config: self.telephony_call_config,
        })
    }
}

