pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendConversationMessageStreamRequest {
    /// Either one `user_message`, which starts a new turn, or `client_tool_result` items, which answer the `required_actions` of the waiting turn and continue it. Calls that they leave out stay pending. Client tools wait with no timeout. While calls are pending, a `user_message` is rejected. A `user_message` can come with `contextual_update` items, at most one per `context_id`. `client_tool_result` items that answer every pending call can come with `end` updates.
    #[serde(default)]
    pub input: Vec<SendConversationMessageStreamRequestInputItem>,
}

impl SendConversationMessageStreamRequest {
    pub fn builder() -> SendConversationMessageStreamRequestBuilder {
        <SendConversationMessageStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendConversationMessageStreamRequestBuilder {
    input: Option<Vec<SendConversationMessageStreamRequestInputItem>>,
}

impl SendConversationMessageStreamRequestBuilder {
    pub fn input(mut self, value: Vec<SendConversationMessageStreamRequestInputItem>) -> Self {
        self.input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendConversationMessageStreamRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](SendConversationMessageStreamRequestBuilder::input)
    pub fn build(self) -> Result<SendConversationMessageStreamRequest, BuildError> {
        Ok(SendConversationMessageStreamRequest {
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
        })
    }
}

