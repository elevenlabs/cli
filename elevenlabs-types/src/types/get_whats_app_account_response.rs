pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWhatsAppAccountResponse {
    #[serde(default)]
    pub business_account_id: String,
    #[serde(default)]
    pub phone_number_id: String,
    #[serde(default)]
    pub business_account_name: String,
    #[serde(default)]
    pub phone_number_name: String,
    #[serde(default)]
    pub phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_messaging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_audio_message_response: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_typing_indicator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_agent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_token_expired: Option<bool>,
}

impl GetWhatsAppAccountResponse {
    pub fn builder() -> GetWhatsAppAccountResponseBuilder {
        <GetWhatsAppAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppAccountResponseBuilder {
    business_account_id: Option<String>,
    phone_number_id: Option<String>,
    business_account_name: Option<String>,
    phone_number_name: Option<String>,
    phone_number: Option<String>,
    assigned_agent_id: Option<String>,
    enable_messaging: Option<bool>,
    enable_audio_message_response: Option<bool>,
    enable_typing_indicator: Option<bool>,
    assigned_agent_name: Option<String>,
    is_token_expired: Option<bool>,
}

impl GetWhatsAppAccountResponseBuilder {
    pub fn business_account_id(mut self, value: impl Into<String>) -> Self {
        self.business_account_id = Some(value.into());
        self
    }

    pub fn phone_number_id(mut self, value: impl Into<String>) -> Self {
        self.phone_number_id = Some(value.into());
        self
    }

    pub fn business_account_name(mut self, value: impl Into<String>) -> Self {
        self.business_account_name = Some(value.into());
        self
    }

    pub fn phone_number_name(mut self, value: impl Into<String>) -> Self {
        self.phone_number_name = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn assigned_agent_id(mut self, value: impl Into<String>) -> Self {
        self.assigned_agent_id = Some(value.into());
        self
    }

    pub fn enable_messaging(mut self, value: bool) -> Self {
        self.enable_messaging = Some(value);
        self
    }

    pub fn enable_audio_message_response(mut self, value: bool) -> Self {
        self.enable_audio_message_response = Some(value);
        self
    }

    pub fn enable_typing_indicator(mut self, value: bool) -> Self {
        self.enable_typing_indicator = Some(value);
        self
    }

    pub fn assigned_agent_name(mut self, value: impl Into<String>) -> Self {
        self.assigned_agent_name = Some(value.into());
        self
    }

    pub fn is_token_expired(mut self, value: bool) -> Self {
        self.is_token_expired = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_account_id`](GetWhatsAppAccountResponseBuilder::business_account_id)
    /// - [`phone_number_id`](GetWhatsAppAccountResponseBuilder::phone_number_id)
    /// - [`business_account_name`](GetWhatsAppAccountResponseBuilder::business_account_name)
    /// - [`phone_number_name`](GetWhatsAppAccountResponseBuilder::phone_number_name)
    /// - [`phone_number`](GetWhatsAppAccountResponseBuilder::phone_number)
    pub fn build(self) -> Result<GetWhatsAppAccountResponse, BuildError> {
        Ok(GetWhatsAppAccountResponse {
            business_account_id: self.business_account_id.ok_or_else(|| BuildError::missing_field("business_account_id"))?,
            phone_number_id: self.phone_number_id.ok_or_else(|| BuildError::missing_field("phone_number_id"))?,
            business_account_name: self.business_account_name.ok_or_else(|| BuildError::missing_field("business_account_name"))?,
            phone_number_name: self.phone_number_name.ok_or_else(|| BuildError::missing_field("phone_number_name"))?,
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
            assigned_agent_id: self.assigned_agent_id,
            enable_messaging: self.enable_messaging,
            enable_audio_message_response: self.enable_audio_message_response,
            enable_typing_indicator: self.enable_typing_indicator,
            assigned_agent_name: self.assigned_agent_name,
            is_token_expired: self.is_token_expired,
        })
    }
}
