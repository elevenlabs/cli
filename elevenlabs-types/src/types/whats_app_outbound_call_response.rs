pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppOutboundCallResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

impl WhatsAppOutboundCallResponse {
    pub fn builder() -> WhatsAppOutboundCallResponseBuilder {
        <WhatsAppOutboundCallResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppOutboundCallResponseBuilder {
    success: Option<bool>,
    message: Option<String>,
    conversation_id: Option<String>,
}

impl WhatsAppOutboundCallResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppOutboundCallResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](WhatsAppOutboundCallResponseBuilder::success)
    /// - [`message`](WhatsAppOutboundCallResponseBuilder::message)
    pub fn build(self) -> Result<WhatsAppOutboundCallResponse, BuildError> {
        Ok(WhatsAppOutboundCallResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            conversation_id: self.conversation_id,
        })
    }
}
