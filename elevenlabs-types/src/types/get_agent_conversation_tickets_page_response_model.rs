pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAgentConversationTicketsPageResponseModel {
    #[serde(default)]
    pub agent_conversation_tickets: Vec<AgentConversationTicketResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl GetAgentConversationTicketsPageResponseModel {
    pub fn builder() -> GetAgentConversationTicketsPageResponseModelBuilder {
        <GetAgentConversationTicketsPageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentConversationTicketsPageResponseModelBuilder {
    agent_conversation_tickets: Option<Vec<AgentConversationTicketResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl GetAgentConversationTicketsPageResponseModelBuilder {
    pub fn agent_conversation_tickets(mut self, value: Vec<AgentConversationTicketResponseModel>) -> Self {
        self.agent_conversation_tickets = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAgentConversationTicketsPageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_conversation_tickets`](GetAgentConversationTicketsPageResponseModelBuilder::agent_conversation_tickets)
    /// - [`has_more`](GetAgentConversationTicketsPageResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetAgentConversationTicketsPageResponseModel, BuildError> {
        Ok(GetAgentConversationTicketsPageResponseModel {
            agent_conversation_tickets: self.agent_conversation_tickets.ok_or_else(|| BuildError::missing_field("agent_conversation_tickets"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
