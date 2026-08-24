pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppConversationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<WhatsAppConversationInfoDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_phone_number_id: Option<String>,
    #[serde(default)]
    pub whatsapp_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awaiting_first_user_message: Option<bool>,
}

impl WhatsAppConversationInfo {
    pub fn builder() -> WhatsAppConversationInfoBuilder {
        <WhatsAppConversationInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppConversationInfoBuilder {
    direction: Option<WhatsAppConversationInfoDirection>,
    whatsapp_phone_number_id: Option<String>,
    whatsapp_user_id: Option<String>,
    awaiting_first_user_message: Option<bool>,
}

impl WhatsAppConversationInfoBuilder {
    pub fn direction(mut self, value: WhatsAppConversationInfoDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn whatsapp_phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_phone_number_id = Some(value.into());
        self
    }

    pub fn whatsapp_user_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_user_id = Some(value.into());
        self
    }

    pub fn awaiting_first_user_message(mut self, value: bool) -> Self {
        self.awaiting_first_user_message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppConversationInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`whatsapp_user_id`](WhatsAppConversationInfoBuilder::whatsapp_user_id)
    pub fn build(self) -> Result<WhatsAppConversationInfo, BuildError> {
        Ok(WhatsAppConversationInfo {
            direction: self.direction,
            whatsapp_phone_number_id: self.whatsapp_phone_number_id,
            whatsapp_user_id: self.whatsapp_user_id.ok_or_else(|| BuildError::missing_field("whatsapp_user_id"))?,
            awaiting_first_user_message: self.awaiting_first_user_message,
        })
    }
}
