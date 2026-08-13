pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPhoneNumberSipTrunkResponseModel {
    /// Phone number
    #[serde(default)]
    pub phone_number: String,
    /// Label for the phone number
    #[serde(default)]
    pub label: String,
    /// This field is deprecated and will be removed in the future. Whether this phone number supports inbound calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inbound: Option<bool>,
    /// This field is deprecated and will be removed in the future. Whether this phone number supports outbound calls
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_outbound: Option<bool>,
    /// The ID of the phone number
    #[serde(default)]
    pub phone_number_id: String,
    /// The agent that is assigned to the phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_agent: Option<PhoneNumberAgentInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_config: Option<GetPhoneNumberOutboundSipTrunkConfigResponseModel>,
    /// Configuration of the Outbound SIP trunk - if configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_trunk: Option<GetPhoneNumberOutboundSipTrunkConfigResponseModel>,
    /// Configuration of the Inbound SIP trunk - if configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_trunk: Option<GetPhoneNumberInboundSipTrunkConfigResponseModel>,
    /// Type of Livekit stack used for this number.
    pub livekit_stack: LivekitStackType,
    /// Whether to store SIP messages for this phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_sip_messages: Option<bool>,
}

impl GetPhoneNumberSipTrunkResponseModel {
    pub fn builder() -> GetPhoneNumberSipTrunkResponseModelBuilder {
        <GetPhoneNumberSipTrunkResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPhoneNumberSipTrunkResponseModelBuilder {
    phone_number: Option<String>,
    label: Option<String>,
    supports_inbound: Option<bool>,
    supports_outbound: Option<bool>,
    phone_number_id: Option<String>,
    assigned_agent: Option<PhoneNumberAgentInfo>,
    provider_config: Option<GetPhoneNumberOutboundSipTrunkConfigResponseModel>,
    outbound_trunk: Option<GetPhoneNumberOutboundSipTrunkConfigResponseModel>,
    inbound_trunk: Option<GetPhoneNumberInboundSipTrunkConfigResponseModel>,
    livekit_stack: Option<LivekitStackType>,
    store_sip_messages: Option<bool>,
}

impl GetPhoneNumberSipTrunkResponseModelBuilder {
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn supports_inbound(mut self, value: bool) -> Self {
        self.supports_inbound = Some(value);
        self
    }

    pub fn supports_outbound(mut self, value: bool) -> Self {
        self.supports_outbound = Some(value);
        self
    }

    pub fn phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.phone_number_id = Some(value.into());
        self
    }

    pub fn assigned_agent(mut self, value: PhoneNumberAgentInfo) -> Self {
        self.assigned_agent = Some(value);
        self
    }

    pub fn provider_config(mut self, value: GetPhoneNumberOutboundSipTrunkConfigResponseModel) -> Self {
        self.provider_config = Some(value);
        self
    }

    pub fn outbound_trunk(mut self, value: GetPhoneNumberOutboundSipTrunkConfigResponseModel) -> Self {
        self.outbound_trunk = Some(value);
        self
    }

    pub fn inbound_trunk(mut self, value: GetPhoneNumberInboundSipTrunkConfigResponseModel) -> Self {
        self.inbound_trunk = Some(value);
        self
    }

    pub fn livekit_stack(mut self, value: LivekitStackType) -> Self {
        self.livekit_stack = Some(value);
        self
    }

    pub fn store_sip_messages(mut self, value: bool) -> Self {
        self.store_sip_messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPhoneNumberSipTrunkResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number`](GetPhoneNumberSipTrunkResponseModelBuilder::phone_number)
    /// - [`label`](GetPhoneNumberSipTrunkResponseModelBuilder::label)
    /// - [`phone_number_id`](GetPhoneNumberSipTrunkResponseModelBuilder::phone_number_id)
    /// - [`livekit_stack`](GetPhoneNumberSipTrunkResponseModelBuilder::livekit_stack)
    pub fn build(self) -> Result<GetPhoneNumberSipTrunkResponseModel, BuildError> {
        Ok(GetPhoneNumberSipTrunkResponseModel {
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            supports_inbound: self.supports_inbound,
            supports_outbound: self.supports_outbound,
            phone_number_id: self.phone_number_id.ok_or_else(|| BuildError::missing_field("phone_number_id"))?,
            assigned_agent: self.assigned_agent,
            provider_config: self.provider_config,
            outbound_trunk: self.outbound_trunk,
            inbound_trunk: self.inbound_trunk,
            livekit_stack: self.livekit_stack.ok_or_else(|| BuildError::missing_field("livekit_stack"))?,
            store_sip_messages: self.store_sip_messages,
        })
    }
}
