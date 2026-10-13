pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentConversationTicketRequestModel {
    /// Conversation this ticket is about.
    #[serde(default)]
    pub conversation_id: String,
    /// One-line headline shown in the triage list. Defaults to one derived from the comments, falling back to the conversation's summary title. Ignored when the comment is added to the conversation's open ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The issue this ticket is about, covering the whole conversation rather than a single turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qa_comment: Option<String>,
    /// Optional turn-level comments on what went wrong.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_comments: Option<Vec<TurnCommentRequestModel>>,
    /// How urgently the ticket needs attention. If the conversation already has an open ticket, it is raised to this priority when lower.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<AgentConversationTicketPriority>,
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
    title: Option<String>,
    qa_comment: Option<String>,
    turn_comments: Option<Vec<TurnCommentRequestModel>>,
    priority: Option<AgentConversationTicketPriority>,
}

impl CreateAgentConversationTicketRequestModelBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
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

    pub fn priority(mut self, value: AgentConversationTicketPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentConversationTicketRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](CreateAgentConversationTicketRequestModelBuilder::conversation_id)
    pub fn build(self) -> Result<CreateAgentConversationTicketRequestModel, BuildError> {
        Ok(CreateAgentConversationTicketRequestModel {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            title: self.title,
            qa_comment: self.qa_comment,
            turn_comments: self.turn_comments,
            priority: self.priority,
        })
    }
}

