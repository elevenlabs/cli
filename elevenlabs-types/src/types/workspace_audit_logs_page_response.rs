pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Paginated workspace audit log response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceAuditLogsPageResponse {
    #[serde(default)]
    pub entries: Vec<WorkspaceAuditLogEntryResponse>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl WorkspaceAuditLogsPageResponse {
    pub fn builder() -> WorkspaceAuditLogsPageResponseBuilder {
        <WorkspaceAuditLogsPageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceAuditLogsPageResponseBuilder {
    entries: Option<Vec<WorkspaceAuditLogEntryResponse>>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl WorkspaceAuditLogsPageResponseBuilder {
    pub fn entries(mut self, value: Vec<WorkspaceAuditLogEntryResponse>) -> Self {
        self.entries = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceAuditLogsPageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entries`](WorkspaceAuditLogsPageResponseBuilder::entries)
    /// - [`has_more`](WorkspaceAuditLogsPageResponseBuilder::has_more)
    pub fn build(self) -> Result<WorkspaceAuditLogsPageResponse, BuildError> {
        Ok(WorkspaceAuditLogsPageResponse {
            entries: self.entries.ok_or_else(|| BuildError::missing_field("entries"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
            next_cursor: self.next_cursor,
        })
    }
}
