pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePost {
    #[serde(default)]
    pub whatsapp_phone_number_id: String,
    #[serde(default)]
    pub whatsapp_user_id: String,
    #[serde(default)]
    pub template_name: String,
    #[serde(default)]
    pub template_language_code: String,
    #[serde(default)]
    pub template_params: Vec<BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostTemplateParamsItem>,
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
}

impl BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePost {
    pub fn builder() -> BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostBuilder {
        <BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostBuilder {
    whatsapp_phone_number_id: Option<String>,
    whatsapp_user_id: Option<String>,
    template_name: Option<String>,
    template_language_code: Option<String>,
    template_params: Option<Vec<BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostTemplateParamsItem>>,
    agent_id: Option<String>,
    conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
}

impl BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostBuilder {
    pub fn whatsapp_phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_phone_number_id = Some(value.into());
        self
    }

    pub fn whatsapp_user_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_user_id = Some(value.into());
        self
    }

    pub fn template_name(mut self, value: impl Into<String>) -> Self {
        self.template_name = Some(value.into());
        self
    }

    pub fn template_language_code(mut self, value: impl Into<String>) -> Self {
        self.template_language_code = Some(value.into());
        self
    }

    pub fn template_params(mut self, value: Vec<BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostTemplateParamsItem>) -> Self {
        self.template_params = Some(value);
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

    /// Consumes the builder and constructs a [`BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`whatsapp_phone_number_id`](BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostBuilder::whatsapp_phone_number_id)
    /// - [`whatsapp_user_id`](BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostBuilder::whatsapp_user_id)
    /// - [`template_name`](BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostBuilder::template_name)
    /// - [`template_language_code`](BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostBuilder::template_language_code)
    /// - [`template_params`](BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostBuilder::template_params)
    /// - [`agent_id`](BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostBuilder::agent_id)
    pub fn build(self) -> Result<BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePost, BuildError> {
        Ok(BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePost {
            whatsapp_phone_number_id: self.whatsapp_phone_number_id.ok_or_else(|| BuildError::missing_field("whatsapp_phone_number_id"))?,
            whatsapp_user_id: self.whatsapp_user_id.ok_or_else(|| BuildError::missing_field("whatsapp_user_id"))?,
            template_name: self.template_name.ok_or_else(|| BuildError::missing_field("template_name"))?,
            template_language_code: self.template_language_code.ok_or_else(|| BuildError::missing_field("template_language_code"))?,
            template_params: self.template_params.ok_or_else(|| BuildError::missing_field("template_params"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            conversation_initiation_client_data: self.conversation_initiation_client_data,
        })
    }
}

