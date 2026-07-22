pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KbExternalSyncJob {
    pub r#type: ExternalSyncProvider,
    #[serde(default)]
    pub folder_id: String,
    #[serde(default)]
    pub integration_connection_id: String,
    pub triggered_by: ExternalSyncJobTrigger,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CrawlStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_type: Option<ExternalSyncJobType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_identified: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_processed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    #[serde(default)]
    pub updated_at: i64,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub created_at: i64,
}

impl KbExternalSyncJob {
    pub fn builder() -> KbExternalSyncJobBuilder {
        <KbExternalSyncJobBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KbExternalSyncJobBuilder {
    r#type: Option<ExternalSyncProvider>,
    folder_id: Option<String>,
    integration_connection_id: Option<String>,
    triggered_by: Option<ExternalSyncJobTrigger>,
    status: Option<CrawlStatus>,
    sync_type: Option<ExternalSyncJobType>,
    items_identified: Option<i64>,
    items_processed: Option<i64>,
    error_message: Option<String>,
    started_at: Option<i64>,
    completed_at: Option<i64>,
    updated_at: Option<i64>,
    id: Option<String>,
    created_at: Option<i64>,
}

impl KbExternalSyncJobBuilder {
    pub fn r#type(mut self, value: ExternalSyncProvider) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn folder_id(mut self, value: impl Into<String>) -> Self {
        self.folder_id = Some(value.into());
        self
    }

    pub fn integration_connection_id(mut self, value: impl Into<String>) -> Self {
        self.integration_connection_id = Some(value.into());
        self
    }

    pub fn triggered_by(mut self, value: ExternalSyncJobTrigger) -> Self {
        self.triggered_by = Some(value);
        self
    }

    pub fn status(mut self, value: CrawlStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn sync_type(mut self, value: ExternalSyncJobType) -> Self {
        self.sync_type = Some(value);
        self
    }

    pub fn items_identified(mut self, value: i64) -> Self {
        self.items_identified = Some(value);
        self
    }

    pub fn items_processed(mut self, value: i64) -> Self {
        self.items_processed = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: i64) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn completed_at(mut self, value: i64) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KbExternalSyncJob`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](KbExternalSyncJobBuilder::r#type)
    /// - [`folder_id`](KbExternalSyncJobBuilder::folder_id)
    /// - [`integration_connection_id`](KbExternalSyncJobBuilder::integration_connection_id)
    /// - [`triggered_by`](KbExternalSyncJobBuilder::triggered_by)
    /// - [`updated_at`](KbExternalSyncJobBuilder::updated_at)
    /// - [`id`](KbExternalSyncJobBuilder::id)
    /// - [`created_at`](KbExternalSyncJobBuilder::created_at)
    pub fn build(self) -> Result<KbExternalSyncJob, BuildError> {
        Ok(KbExternalSyncJob {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            folder_id: self.folder_id.ok_or_else(|| BuildError::missing_field("folder_id"))?,
            integration_connection_id: self.integration_connection_id.ok_or_else(|| BuildError::missing_field("integration_connection_id"))?,
            triggered_by: self.triggered_by.ok_or_else(|| BuildError::missing_field("triggered_by"))?,
            status: self.status,
            sync_type: self.sync_type,
            items_identified: self.items_identified,
            items_processed: self.items_processed,
            error_message: self.error_message,
            started_at: self.started_at,
            completed_at: self.completed_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
