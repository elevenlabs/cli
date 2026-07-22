pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWhatsAppAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_messaging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_audio_message_response: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_typing_indicator: Option<bool>,
}

impl UpdateWhatsAppAccountRequest {
    pub fn builder() -> UpdateWhatsAppAccountRequestBuilder {
        <UpdateWhatsAppAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWhatsAppAccountRequestBuilder {
    assigned_agent_id: Option<String>,
    enable_messaging: Option<bool>,
    enable_audio_message_response: Option<bool>,
    enable_typing_indicator: Option<bool>,
}

impl UpdateWhatsAppAccountRequestBuilder {
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

    /// Consumes the builder and constructs a [`UpdateWhatsAppAccountRequest`].
    pub fn build(self) -> Result<UpdateWhatsAppAccountRequest, BuildError> {
        Ok(UpdateWhatsAppAccountRequest {
            assigned_agent_id: self.assigned_agent_id,
            enable_messaging: self.enable_messaging,
            enable_audio_message_response: self.enable_audio_message_response,
            enable_typing_indicator: self.enable_typing_indicator,
        })
    }
}

