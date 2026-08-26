pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsTriageTicketsListQueryRequest {
    /// How many agent conversation tickets to return. Can not exceed 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Filter tickets by conversation id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Filter tickets by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgentConversationTicketStatus>,
    /// Filter tickets by how they were raised (qa, agent, manual). Repeat the parameter to filter by multiple sources.
    #[serde(default)]
    pub sources: Vec<Option<AgentConversationTicketSource>>,
    /// Filter tickets by creator. Use 'agent' for agent-raised tickets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    /// Filter tickets by assignee. Use 'unassigned' for tickets with no assignee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_user_id: Option<String>,
    /// Filter clusters by issue type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<AgentConversationTicketIssueType>,
    /// Filter tickets by an exact label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsTriageTicketsListQueryRequest {
    pub fn builder() -> AgentsTriageTicketsListQueryRequestBuilder {
        <AgentsTriageTicketsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsTriageTicketsListQueryRequestBuilder {
    page_size: Option<i64>,
    conversation_id: Option<String>,
    status: Option<AgentConversationTicketStatus>,
    sources: Option<Vec<Option<AgentConversationTicketSource>>>,
    owner_user_id: Option<String>,
    assignee_user_id: Option<String>,
    issue_type: Option<AgentConversationTicketIssueType>,
    label: Option<String>,
    cursor: Option<String>,
}

impl AgentsTriageTicketsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: AgentConversationTicketStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<Option<AgentConversationTicketSource>>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn owner_user_id(mut self, value: impl Into<String>) -> Self {
        self.owner_user_id = Some(value.into());
        self
    }

    pub fn assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.assignee_user_id = Some(value.into());
        self
    }

    pub fn issue_type(mut self, value: AgentConversationTicketIssueType) -> Self {
        self.issue_type = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsTriageTicketsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sources`](AgentsTriageTicketsListQueryRequestBuilder::sources)
    pub fn build(self) -> Result<AgentsTriageTicketsListQueryRequest, BuildError> {
        Ok(AgentsTriageTicketsListQueryRequest {
            page_size: self.page_size,
            conversation_id: self.conversation_id,
            status: self.status,
            sources: self.sources.ok_or_else(|| BuildError::missing_field("sources"))?,
            owner_user_id: self.owner_user_id,
            assignee_user_id: self.assignee_user_id,
            issue_type: self.issue_type,
            label: self.label,
            cursor: self.cursor,
        })
    }
}

