pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExotelOutboundCallResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Exotel CallSid for the originated call
    #[serde(rename = "callSid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<String>,
}

impl ExotelOutboundCallResponse {
    pub fn builder() -> ExotelOutboundCallResponseBuilder {
        <ExotelOutboundCallResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExotelOutboundCallResponseBuilder {
    success: Option<bool>,
    message: Option<String>,
    conversation_id: Option<String>,
    call_sid: Option<String>,
}

impl ExotelOutboundCallResponseBuilder {
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

    pub fn call_sid(mut self, value: impl Into<String>) -> Self {
        self.call_sid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExotelOutboundCallResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ExotelOutboundCallResponseBuilder::success)
    /// - [`message`](ExotelOutboundCallResponseBuilder::message)
    pub fn build(self) -> Result<ExotelOutboundCallResponse, BuildError> {
        Ok(ExotelOutboundCallResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            conversation_id: self.conversation_id,
            call_sid: self.call_sid,
        })
    }
}
