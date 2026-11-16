pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentConversationResponse {
    #[serde(default)]
    pub conversation_id: String,
}

impl CreateAgentConversationResponse {
    pub fn builder() -> CreateAgentConversationResponseBuilder {
        <CreateAgentConversationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentConversationResponseBuilder {
    conversation_id: Option<String>,
}

impl CreateAgentConversationResponseBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentConversationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](CreateAgentConversationResponseBuilder::conversation_id)
    pub fn build(self) -> Result<CreateAgentConversationResponse, BuildError> {
        Ok(CreateAgentConversationResponse {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
        })
    }
}
