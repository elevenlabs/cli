pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Audit log entry with Firestore document ID for API responses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspaceAuditLogEntryResponse {
    /// Event metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Event time in milliseconds since epoch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Activity ID
    pub activity_id: WorkspaceAuditLogEntryResponseActivityId,
    /// Activity name
    #[serde(default)]
    pub activity_name: String,
    /// Event category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    /// Category UID for IAM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_uid: Option<i64>,
    /// Event class name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// Event class UID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_uid: Option<i64>,
    /// Severity level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_id: Option<SeverityId>,
    /// Status of the action
    #[serde(default)]
    pub status_id: StatusId,
    /// Actor performing the action
    #[serde(default)]
    pub actor: ActorModel,
    /// Device information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceModel>,
    /// HTTP request details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_request: Option<HttpRequestModel>,
    /// Human-readable event description
    #[serde(default)]
    pub message: String,
    /// Attributes not mapped to OCSF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmapped: Option<HashMap<String, serde_json::Value>>,
    /// Firestore document ID
    #[serde(default)]
    pub id: String,
    /// Event time in human-readable RFC 3339 format, derived from 'time'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_dt: Option<String>,
    /// OCSF type_uid is class_uid * 100 + activity_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_uid: Option<i64>,
    /// OCSF type_name combines class_name and activity_name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

impl WorkspaceAuditLogEntryResponse {
    pub fn builder() -> WorkspaceAuditLogEntryResponseBuilder {
        <WorkspaceAuditLogEntryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceAuditLogEntryResponseBuilder {
    metadata: Option<HashMap<String, serde_json::Value>>,
    time: Option<i64>,
    activity_id: Option<WorkspaceAuditLogEntryResponseActivityId>,
    activity_name: Option<String>,
    category_name: Option<String>,
    category_uid: Option<i64>,
    class_name: Option<String>,
    class_uid: Option<i64>,
    severity_id: Option<SeverityId>,
    status_id: Option<StatusId>,
    actor: Option<ActorModel>,
    device: Option<DeviceModel>,
    http_request: Option<HttpRequestModel>,
    message: Option<String>,
    unmapped: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    time_dt: Option<String>,
    type_uid: Option<i64>,
    type_name: Option<String>,
}

impl WorkspaceAuditLogEntryResponseBuilder {
    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn time(mut self, value: i64) -> Self {
        self.time = Some(value);
        self
    }

    pub fn activity_id(mut self, value: WorkspaceAuditLogEntryResponseActivityId) -> Self {
        self.activity_id = Some(value);
        self
    }

    pub fn activity_name(mut self, value: impl Into<String>) -> Self {
        self.activity_name = Some(value.into());
        self
    }

    pub fn category_name(mut self, value: impl Into<String>) -> Self {
        self.category_name = Some(value.into());
        self
    }

    pub fn category_uid(mut self, value: i64) -> Self {
        self.category_uid = Some(value);
        self
    }

    pub fn class_name(mut self, value: impl Into<String>) -> Self {
        self.class_name = Some(value.into());
        self
    }

    pub fn class_uid(mut self, value: i64) -> Self {
        self.class_uid = Some(value);
        self
    }

    pub fn severity_id(mut self, value: SeverityId) -> Self {
        self.severity_id = Some(value);
        self
    }

    pub fn status_id(mut self, value: StatusId) -> Self {
        self.status_id = Some(value);
        self
    }

    pub fn actor(mut self, value: ActorModel) -> Self {
        self.actor = Some(value);
        self
    }

    pub fn device(mut self, value: DeviceModel) -> Self {
        self.device = Some(value);
        self
    }

    pub fn http_request(mut self, value: HttpRequestModel) -> Self {
        self.http_request = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn unmapped(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.unmapped = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn time_dt(mut self, value: impl Into<String>) -> Self {
        self.time_dt = Some(value.into());
        self
    }

    pub fn type_uid(mut self, value: i64) -> Self {
        self.type_uid = Some(value);
        self
    }

    pub fn type_name(mut self, value: impl Into<String>) -> Self {
        self.type_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceAuditLogEntryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`activity_id`](WorkspaceAuditLogEntryResponseBuilder::activity_id)
    /// - [`activity_name`](WorkspaceAuditLogEntryResponseBuilder::activity_name)
    /// - [`status_id`](WorkspaceAuditLogEntryResponseBuilder::status_id)
    /// - [`actor`](WorkspaceAuditLogEntryResponseBuilder::actor)
    /// - [`message`](WorkspaceAuditLogEntryResponseBuilder::message)
    /// - [`id`](WorkspaceAuditLogEntryResponseBuilder::id)
    pub fn build(self) -> Result<WorkspaceAuditLogEntryResponse, BuildError> {
        Ok(WorkspaceAuditLogEntryResponse {
            metadata: self.metadata,
            time: self.time,
            activity_id: self.activity_id.ok_or_else(|| BuildError::missing_field("activity_id"))?,
            activity_name: self.activity_name.ok_or_else(|| BuildError::missing_field("activity_name"))?,
            category_name: self.category_name,
            category_uid: self.category_uid,
            class_name: self.class_name,
            class_uid: self.class_uid,
            severity_id: self.severity_id,
            status_id: self.status_id.ok_or_else(|| BuildError::missing_field("status_id"))?,
            actor: self.actor.ok_or_else(|| BuildError::missing_field("actor"))?,
            device: self.device,
            http_request: self.http_request,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            unmapped: self.unmapped,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            time_dt: self.time_dt,
            type_uid: self.type_uid,
            type_name: self.type_name,
        })
    }
}
