pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OutboundCallRecipient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
}

impl OutboundCallRecipient {
    pub fn builder() -> OutboundCallRecipientBuilder {
        <OutboundCallRecipientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutboundCallRecipientBuilder {
    id: Option<String>,
    phone_number: Option<String>,
    whatsapp_user_id: Option<String>,
    conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
}

impl OutboundCallRecipientBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn whatsapp_user_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_user_id = Some(value.into());
        self
    }

    pub fn conversation_initiation_client_data(mut self, value: ConversationInitiationClientDataRequestInput) -> Self {
        self.conversation_initiation_client_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OutboundCallRecipient`].
    pub fn build(self) -> Result<OutboundCallRecipient, BuildError> {
        Ok(OutboundCallRecipient {
            id: self.id,
            phone_number: self.phone_number,
            whatsapp_user_id: self.whatsapp_user_id,
            conversation_initiation_client_data: self.conversation_initiation_client_data,
        })
    }
}
