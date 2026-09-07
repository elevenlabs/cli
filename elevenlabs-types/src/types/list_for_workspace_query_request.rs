pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_for_workspace
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListForWorkspaceQueryRequest {
    /// How many agent conversation tickets to return. Can not exceed 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Filter tickets by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgentConversationTicketStatus>,
    /// Filter tickets by assignee. Use 'unassigned' for tickets with no assignee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_user_id: Option<String>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListForWorkspaceQueryRequest {
    pub fn builder() -> ListForWorkspaceQueryRequestBuilder {
        <ListForWorkspaceQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListForWorkspaceQueryRequestBuilder {
    page_size: Option<i64>,
    status: Option<AgentConversationTicketStatus>,
    assignee_user_id: Option<String>,
    cursor: Option<String>,
}

impl ListForWorkspaceQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
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

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListForWorkspaceQueryRequest`].
    pub fn build(self) -> Result<ListForWorkspaceQueryRequest, BuildError> {
        Ok(ListForWorkspaceQueryRequest {
            page_size: self.page_size,
            status: self.status,
            assignee_user_id: self.assignee_user_id,
            cursor: self.cursor,
        })
    }
}

