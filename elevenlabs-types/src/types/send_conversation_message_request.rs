pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendConversationMessageRequest {
    /// Either one `user_message`, which starts a new turn, or `client_tool_result` items, which answer the `required_actions` of the waiting turn and continue it. Calls that they leave out stay pending. Client tools wait with no timeout. While calls are pending, a `user_message` is rejected. A `user_message` can come with `contextual_update` items, at most one per `context_id`. `client_tool_result` items that answer every pending call can come with `end` updates.
    #[serde(default)]
    pub input: Vec<SendConversationMessageRequestInputItem>,
    /// If true, return a 202 handle after the turn is admitted instead of waiting for generation to finish.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
}

impl SendConversationMessageRequest {
    pub fn builder() -> SendConversationMessageRequestBuilder {
        <SendConversationMessageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendConversationMessageRequestBuilder {
    input: Option<Vec<SendConversationMessageRequestInputItem>>,
    background: Option<bool>,
}

impl SendConversationMessageRequestBuilder {
    pub fn input(mut self, value: Vec<SendConversationMessageRequestInputItem>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn background(mut self, value: bool) -> Self {
        self.background = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendConversationMessageRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](SendConversationMessageRequestBuilder::input)
    pub fn build(self) -> Result<SendConversationMessageRequest, BuildError> {
        Ok(SendConversationMessageRequest {
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
            background: self.background,
        })
    }
}

