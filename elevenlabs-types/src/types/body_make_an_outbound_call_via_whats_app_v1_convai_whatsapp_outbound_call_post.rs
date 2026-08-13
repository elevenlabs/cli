pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPost {
    #[serde(default)]
    pub whatsapp_phone_number_id: String,
    #[serde(default)]
    pub whatsapp_user_id: String,
    #[serde(default)]
    pub whatsapp_call_permission_request_template_name: String,
    #[serde(default)]
    pub whatsapp_call_permission_request_template_language_code: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
}

impl BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPost {
    pub fn builder() -> BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPostBuilder {
        <BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPostBuilder {
    whatsapp_phone_number_id: Option<String>,
    whatsapp_user_id: Option<String>,
    whatsapp_call_permission_request_template_name: Option<String>,
    whatsapp_call_permission_request_template_language_code: Option<String>,
    agent_id: Option<String>,
    conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
}

impl BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPostBuilder {
    pub fn whatsapp_phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_phone_number_id = Some(value.into());
        self
    }

    pub fn whatsapp_user_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_user_id = Some(value.into());
        self
    }

    pub fn whatsapp_call_permission_request_template_name(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_call_permission_request_template_name = Some(value.into());
        self
    }

    pub fn whatsapp_call_permission_request_template_language_code(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_call_permission_request_template_language_code = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn conversation_initiation_client_data(mut self, value: ConversationInitiationClientDataRequestInput) -> Self {
        self.conversation_initiation_client_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`whatsapp_phone_number_id`](BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPostBuilder::whatsapp_phone_number_id)
    /// - [`whatsapp_user_id`](BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPostBuilder::whatsapp_user_id)
    /// - [`whatsapp_call_permission_request_template_name`](BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPostBuilder::whatsapp_call_permission_request_template_name)
    /// - [`whatsapp_call_permission_request_template_language_code`](BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPostBuilder::whatsapp_call_permission_request_template_language_code)
    /// - [`agent_id`](BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPostBuilder::agent_id)
    pub fn build(self) -> Result<BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPost, BuildError> {
        Ok(BodyMakeAnOutboundCallViaWhatsAppV1ConvaiWhatsappOutboundCallPost {
            whatsapp_phone_number_id: self.whatsapp_phone_number_id.ok_or_else(|| BuildError::missing_field("whatsapp_phone_number_id"))?,
            whatsapp_user_id: self.whatsapp_user_id.ok_or_else(|| BuildError::missing_field("whatsapp_user_id"))?,
            whatsapp_call_permission_request_template_name: self.whatsapp_call_permission_request_template_name.ok_or_else(|| BuildError::missing_field("whatsapp_call_permission_request_template_name"))?,
            whatsapp_call_permission_request_template_language_code: self.whatsapp_call_permission_request_template_language_code.ok_or_else(|| BuildError::missing_field("whatsapp_call_permission_request_template_language_code"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            conversation_initiation_client_data: self.conversation_initiation_client_data,
        })
    }
}

