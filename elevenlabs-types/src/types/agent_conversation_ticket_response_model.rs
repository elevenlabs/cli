pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentConversationTicketResponseModel {
    #[serde(default)]
    pub agentqa_ticket_id: String,
    #[serde(default)]
    pub workspace_id: String,
    #[serde(default)]
    pub owner_user_id: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub needs_clustering: bool,
    /// One-line headline for the ticket. None only on tickets created before titles existed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<AgentConversationTicketIssueType>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub conversation_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_unix_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_unix_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qa_comment: Option<String>,
    #[serde(default)]
    pub ticket_comments: Vec<TicketCommentResponseModel>,
    #[serde(default)]
    pub turn_comments: Vec<TurnCommentResponseModel>,
    pub status: AgentConversationTicketStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<AgentConversationTicketPriority>,
    #[serde(default)]
    pub priority_changes: Vec<TicketPriorityChangeResponseModel>,
    #[serde(default)]
    pub status_changes: Vec<TicketStatusChangeResponseModel>,
    /// The ticket this one was merged into, set while its status is 'merged'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_into_ticket_id: Option<String>,
    pub source: AgentConversationTicketSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_user_id: Option<String>,
    #[serde(default)]
    pub created_at_unix_secs: i64,
    #[serde(default)]
    pub updated_at_unix_secs: i64,
    /// Where the list's `search` query matched. Set only on searched lists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_match: Option<TicketSearchMatchResponseModel>,
}

impl AgentConversationTicketResponseModel {
    pub fn builder() -> AgentConversationTicketResponseModelBuilder {
        <AgentConversationTicketResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentConversationTicketResponseModelBuilder {
    agentqa_ticket_id: Option<String>,
    workspace_id: Option<String>,
    owner_user_id: Option<String>,
    agent_id: Option<String>,
    needs_clustering: Option<bool>,
    title: Option<String>,
    issue_type: Option<AgentConversationTicketIssueType>,
    labels: Option<Vec<String>>,
    conversation_ids: Option<Vec<String>>,
    first_seen_unix_secs: Option<i64>,
    last_seen_unix_secs: Option<i64>,
    qa_comment: Option<String>,
    ticket_comments: Option<Vec<TicketCommentResponseModel>>,
    turn_comments: Option<Vec<TurnCommentResponseModel>>,
    status: Option<AgentConversationTicketStatus>,
    priority: Option<AgentConversationTicketPriority>,
    priority_changes: Option<Vec<TicketPriorityChangeResponseModel>>,
    status_changes: Option<Vec<TicketStatusChangeResponseModel>>,
    merged_into_ticket_id: Option<String>,
    source: Option<AgentConversationTicketSource>,
    assignee_user_id: Option<String>,
    created_at_unix_secs: Option<i64>,
    updated_at_unix_secs: Option<i64>,
    search_match: Option<TicketSearchMatchResponseModel>,
}

impl AgentConversationTicketResponseModelBuilder {
    pub fn agentqa_ticket_id(mut self, value: impl Into<String>) -> Self {
        self.agentqa_ticket_id = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn owner_user_id(mut self, value: impl Into<String>) -> Self {
        self.owner_user_id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn needs_clustering(mut self, value: bool) -> Self {
        self.needs_clustering = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn issue_type(mut self, value: AgentConversationTicketIssueType) -> Self {
        self.issue_type = Some(value);
        self
    }

    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn conversation_ids(mut self, value: Vec<String>) -> Self {
        self.conversation_ids = Some(value);
        self
    }

    pub fn first_seen_unix_secs(mut self, value: i64) -> Self {
        self.first_seen_unix_secs = Some(value);
        self
    }

    pub fn last_seen_unix_secs(mut self, value: i64) -> Self {
        self.last_seen_unix_secs = Some(value);
        self
    }

    pub fn qa_comment(mut self, value: impl Into<String>) -> Self {
        self.qa_comment = Some(value.into());
        self
    }

    pub fn ticket_comments(mut self, value: Vec<TicketCommentResponseModel>) -> Self {
        self.ticket_comments = Some(value);
        self
    }

    pub fn turn_comments(mut self, value: Vec<TurnCommentResponseModel>) -> Self {
        self.turn_comments = Some(value);
        self
    }

    pub fn status(mut self, value: AgentConversationTicketStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn priority(mut self, value: AgentConversationTicketPriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn priority_changes(mut self, value: Vec<TicketPriorityChangeResponseModel>) -> Self {
        self.priority_changes = Some(value);
        self
    }

    pub fn status_changes(mut self, value: Vec<TicketStatusChangeResponseModel>) -> Self {
        self.status_changes = Some(value);
        self
    }

    pub fn merged_into_ticket_id(mut self, value: impl Into<String>) -> Self {
        self.merged_into_ticket_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: AgentConversationTicketSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.assignee_user_id = Some(value.into());
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn updated_at_unix_secs(mut self, value: i64) -> Self {
        self.updated_at_unix_secs = Some(value);
        self
    }

    pub fn search_match(mut self, value: TicketSearchMatchResponseModel) -> Self {
        self.search_match = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentConversationTicketResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agentqa_ticket_id`](AgentConversationTicketResponseModelBuilder::agentqa_ticket_id)
    /// - [`workspace_id`](AgentConversationTicketResponseModelBuilder::workspace_id)
    /// - [`owner_user_id`](AgentConversationTicketResponseModelBuilder::owner_user_id)
    /// - [`agent_id`](AgentConversationTicketResponseModelBuilder::agent_id)
    /// - [`needs_clustering`](AgentConversationTicketResponseModelBuilder::needs_clustering)
    /// - [`labels`](AgentConversationTicketResponseModelBuilder::labels)
    /// - [`conversation_ids`](AgentConversationTicketResponseModelBuilder::conversation_ids)
    /// - [`ticket_comments`](AgentConversationTicketResponseModelBuilder::ticket_comments)
    /// - [`turn_comments`](AgentConversationTicketResponseModelBuilder::turn_comments)
    /// - [`status`](AgentConversationTicketResponseModelBuilder::status)
    /// - [`priority_changes`](AgentConversationTicketResponseModelBuilder::priority_changes)
    /// - [`status_changes`](AgentConversationTicketResponseModelBuilder::status_changes)
    /// - [`source`](AgentConversationTicketResponseModelBuilder::source)
    /// - [`created_at_unix_secs`](AgentConversationTicketResponseModelBuilder::created_at_unix_secs)
    /// - [`updated_at_unix_secs`](AgentConversationTicketResponseModelBuilder::updated_at_unix_secs)
    pub fn build(self) -> Result<AgentConversationTicketResponseModel, BuildError> {
        Ok(AgentConversationTicketResponseModel {
            agentqa_ticket_id: self.agentqa_ticket_id.ok_or_else(|| BuildError::missing_field("agentqa_ticket_id"))?,
            workspace_id: self.workspace_id.ok_or_else(|| BuildError::missing_field("workspace_id"))?,
            owner_user_id: self.owner_user_id.ok_or_else(|| BuildError::missing_field("owner_user_id"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            needs_clustering: self.needs_clustering.ok_or_else(|| BuildError::missing_field("needs_clustering"))?,
            title: self.title,
            issue_type: self.issue_type,
            labels: self.labels.ok_or_else(|| BuildError::missing_field("labels"))?,
            conversation_ids: self.conversation_ids.ok_or_else(|| BuildError::missing_field("conversation_ids"))?,
            first_seen_unix_secs: self.first_seen_unix_secs,
            last_seen_unix_secs: self.last_seen_unix_secs,
            qa_comment: self.qa_comment,
            ticket_comments: self.ticket_comments.ok_or_else(|| BuildError::missing_field("ticket_comments"))?,
            turn_comments: self.turn_comments.ok_or_else(|| BuildError::missing_field("turn_comments"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            priority: self.priority,
            priority_changes: self.priority_changes.ok_or_else(|| BuildError::missing_field("priority_changes"))?,
            status_changes: self.status_changes.ok_or_else(|| BuildError::missing_field("status_changes"))?,
            merged_into_ticket_id: self.merged_into_ticket_id,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            assignee_user_id: self.assignee_user_id,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            updated_at_unix_secs: self.updated_at_unix_secs.ok_or_else(|| BuildError::missing_field("updated_at_unix_secs"))?,
            search_match: self.search_match,
        })
    }
}
