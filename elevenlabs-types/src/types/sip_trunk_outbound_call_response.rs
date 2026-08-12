pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SipTrunkOutboundCallResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_call_id: Option<String>,
}

impl SipTrunkOutboundCallResponse {
    pub fn builder() -> SipTrunkOutboundCallResponseBuilder {
        <SipTrunkOutboundCallResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipTrunkOutboundCallResponseBuilder {
    success: Option<bool>,
    message: Option<String>,
    conversation_id: Option<String>,
    sip_call_id: Option<String>,
}

impl SipTrunkOutboundCallResponseBuilder {
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

    pub fn sip_call_id(mut self, value: impl Into<String>) -> Self {
        self.sip_call_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SipTrunkOutboundCallResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](SipTrunkOutboundCallResponseBuilder::success)
    /// - [`message`](SipTrunkOutboundCallResponseBuilder::message)
    pub fn build(self) -> Result<SipTrunkOutboundCallResponse, BuildError> {
        Ok(SipTrunkOutboundCallResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            conversation_id: self.conversation_id,
            sip_call_id: self.sip_call_id,
        })
    }
}
