pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsAppOutboundMessageResponse {
    #[serde(default)]
    pub conversation_id: String,
}

impl WhatsAppOutboundMessageResponse {
    pub fn builder() -> WhatsAppOutboundMessageResponseBuilder {
        <WhatsAppOutboundMessageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsAppOutboundMessageResponseBuilder {
    conversation_id: Option<String>,
}

impl WhatsAppOutboundMessageResponseBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WhatsAppOutboundMessageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](WhatsAppOutboundMessageResponseBuilder::conversation_id)
    pub fn build(self) -> Result<WhatsAppOutboundMessageResponse, BuildError> {
        Ok(WhatsAppOutboundMessageResponse {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
        })
    }
}
