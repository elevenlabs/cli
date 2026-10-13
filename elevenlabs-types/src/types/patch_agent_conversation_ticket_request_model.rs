pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchAgentConversationTicketRequestModel {
    /// If provided, updates the ticket title. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// If provided, updates the ticket status. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgentConversationTicketStatus>,
    /// If provided, updates who is responsible for resolving this ticket. Must be a workspace member with at least viewer access to the agent. Pass null to unassign. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_user_id: Option<String>,
    /// If provided, updates how urgently the ticket needs attention. Pass null to clear it. Omit to leave unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<AgentConversationTicketPriority>,
}

impl PatchAgentConversationTicketRequestModel {
    pub fn builder() -> PatchAgentConversationTicketRequestModelBuilder {
        <PatchAgentConversationTicketRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchAgentConversationTicketRequestModelBuilder {
    title: Option<String>,
    status: Option<AgentConversationTicketStatus>,
    assignee_user_id: Option<String>,
    priority: Option<AgentConversationTicketPriority>,
}

impl PatchAgentConversationTicketRequestModelBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn status(mut self, value: AgentConversationTicketStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.assignee_user_id = Some(value.into());
        self
    }

    pub fn priority(mut self, value: AgentConversationTicketPriority) -> Self {
        self.priority = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatchAgentConversationTicketRequestModel`].
    pub fn build(self) -> Result<PatchAgentConversationTicketRequestModel, BuildError> {
        Ok(PatchAgentConversationTicketRequestModel {
            title: self.title,
            status: self.status,
            assignee_user_id: self.assignee_user_id,
            priority: self.priority,
        })
    }
}

