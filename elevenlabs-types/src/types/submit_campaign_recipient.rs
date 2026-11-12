pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubmitCampaignRecipient {
    /// The number to call, in E.164 format such as `+14155550100`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// More numbers to call, in order after `phone_number`. The lead moves on to the next one when calls to its current number keep failing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
    /// The contact's WhatsApp user ID. WhatsApp campaigns use it instead of the phone numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_user_id: Option<String>,
    /// Your identifier for the recipient, returned as its lead's `recipient_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Dynamic variables and overrides for this recipient's calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
}

impl SubmitCampaignRecipient {
    pub fn builder() -> SubmitCampaignRecipientBuilder {
        <SubmitCampaignRecipientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitCampaignRecipientBuilder {
    phone_number: Option<String>,
    phone_numbers: Option<Vec<String>>,
    whatsapp_user_id: Option<String>,
    id: Option<String>,
    conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestInput>,
}

impl SubmitCampaignRecipientBuilder {
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn phone_numbers(mut self, value: Vec<String>) -> Self {
        self.phone_numbers = Some(value);
        self
    }

    pub fn whatsapp_user_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_user_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn conversation_initiation_client_data(mut self, value: ConversationInitiationClientDataRequestInput) -> Self {
        self.conversation_initiation_client_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitCampaignRecipient`].
    pub fn build(self) -> Result<SubmitCampaignRecipient, BuildError> {
        Ok(SubmitCampaignRecipient {
            phone_number: self.phone_number,
            phone_numbers: self.phone_numbers,
            whatsapp_user_id: self.whatsapp_user_id,
            id: self.id,
            conversation_initiation_client_data: self.conversation_initiation_client_data,
        })
    }
}
