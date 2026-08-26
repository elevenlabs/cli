pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentConversationTicketRequestModel {
    /// Conversation this ticket is about.
    #[serde(default)]
    pub conversation_id: String,
    /// The QA finding covering the whole conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qa_comment: Option<String>,
    /// Optional turn-level comments on what went wrong.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_comments: Option<Vec<TurnCommentRequestModel>>,
}

impl CreateAgentConversationTicketRequestModel {
    pub fn builder() -> CreateAgentConversationTicketRequestModelBuilder {
        <CreateAgentConversationTicketRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentConversationTicketRequestModelBuilder {
    conversation_id: Option<String>,
    qa_comment: Option<String>,
    turn_comments: Option<Vec<TurnCommentRequestModel>>,
}

impl CreateAgentConversationTicketRequestModelBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn qa_comment(mut self, value: impl Into<String>) -> Self {
        self.qa_comment = Some(value.into());
        self
    }

    pub fn turn_comments(mut self, value: Vec<TurnCommentRequestModel>) -> Self {
        self.turn_comments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentConversationTicketRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](CreateAgentConversationTicketRequestModelBuilder::conversation_id)
    pub fn build(self) -> Result<CreateAgentConversationTicketRequestModel, BuildError> {
        Ok(CreateAgentConversationTicketRequestModel {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            qa_comment: self.qa_comment,
            turn_comments: self.turn_comments,
        })
    }
}

