pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceAuditLogsListQueryRequest {
    /// Maximum number of entries per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor for the next page (from previous response)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Only include entries at or after this time (ms since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_from_unix_ms: Option<i64>,
    /// Only include entries at or before this time (ms since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_unix_ms: Option<i64>,
    /// Filter by actor user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_uid: Option<String>,
    /// Filter by OCSF event class name (e.g. Account Change)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// Filter by audit activity name (e.g. Subscription Creation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
}

impl WorkspaceAuditLogsListQueryRequest {
    pub fn builder() -> WorkspaceAuditLogsListQueryRequestBuilder {
        <WorkspaceAuditLogsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceAuditLogsListQueryRequestBuilder {
    limit: Option<i64>,
    cursor: Option<String>,
    time_from_unix_ms: Option<i64>,
    time_to_unix_ms: Option<i64>,
    actor_uid: Option<String>,
    class_name: Option<String>,
    activity_name: Option<String>,
}

impl WorkspaceAuditLogsListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn time_from_unix_ms(mut self, value: i64) -> Self {
        self.time_from_unix_ms = Some(value);
        self
    }

    pub fn time_to_unix_ms(mut self, value: i64) -> Self {
        self.time_to_unix_ms = Some(value);
        self
    }

    pub fn actor_uid(mut self, value: impl Into<String>) -> Self {
        self.actor_uid = Some(value.into());
        self
    }

    pub fn class_name(mut self, value: impl Into<String>) -> Self {
        self.class_name = Some(value.into());
        self
    }

    pub fn activity_name(mut self, value: impl Into<String>) -> Self {
        self.activity_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceAuditLogsListQueryRequest`].
    pub fn build(self) -> Result<WorkspaceAuditLogsListQueryRequest, BuildError> {
        Ok(WorkspaceAuditLogsListQueryRequest {
            limit: self.limit,
            cursor: self.cursor,
            time_from_unix_ms: self.time_from_unix_ms,
            time_to_unix_ms: self.time_to_unix_ms,
            actor_uid: self.actor_uid,
            class_name: self.class_name,
            activity_name: self.activity_name,
        })
    }
}

