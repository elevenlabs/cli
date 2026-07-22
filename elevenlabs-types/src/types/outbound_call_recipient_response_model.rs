pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutboundCallRecipientResponseModel {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_user_id: Option<String>,
    pub status: BatchCallRecipientStatus,
    #[serde(default)]
    pub created_at_unix: i64,
    #[serde(default)]
    pub updated_at_unix: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<ConversationInitiationClientDataInternal>,
}

impl OutboundCallRecipientResponseModel {
    pub fn builder() -> OutboundCallRecipientResponseModelBuilder {
        <OutboundCallRecipientResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutboundCallRecipientResponseModelBuilder {
    id: Option<String>,
    phone_number: Option<String>,
    whatsapp_user_id: Option<String>,
    status: Option<BatchCallRecipientStatus>,
    created_at_unix: Option<i64>,
    updated_at_unix: Option<i64>,
    conversation_id: Option<String>,
    conversation_initiation_client_data: Option<ConversationInitiationClientDataInternal>,
}

impl OutboundCallRecipientResponseModelBuilder {
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

    pub fn status(mut self, value: BatchCallRecipientStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn updated_at_unix(mut self, value: i64) -> Self {
        self.updated_at_unix = Some(value);
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn conversation_initiation_client_data(mut self, value: ConversationInitiationClientDataInternal) -> Self {
        self.conversation_initiation_client_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OutboundCallRecipientResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OutboundCallRecipientResponseModelBuilder::id)
    /// - [`status`](OutboundCallRecipientResponseModelBuilder::status)
    /// - [`created_at_unix`](OutboundCallRecipientResponseModelBuilder::created_at_unix)
    /// - [`updated_at_unix`](OutboundCallRecipientResponseModelBuilder::updated_at_unix)
    pub fn build(self) -> Result<OutboundCallRecipientResponseModel, BuildError> {
        Ok(OutboundCallRecipientResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            phone_number: self.phone_number,
            whatsapp_user_id: self.whatsapp_user_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            updated_at_unix: self.updated_at_unix.ok_or_else(|| BuildError::missing_field("updated_at_unix"))?,
            conversation_id: self.conversation_id,
            conversation_initiation_client_data: self.conversation_initiation_client_data,
        })
    }
}
