pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// In-flight/last refresh state for an externally-synced KB file.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileRefreshStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CrawlStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enqueued_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synced_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl FileRefreshStatus {
    pub fn builder() -> FileRefreshStatusBuilder {
        <FileRefreshStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileRefreshStatusBuilder {
    status: Option<CrawlStatus>,
    enqueued_at: Option<i64>,
    started_at: Option<i64>,
    completed_at: Option<i64>,
    last_synced_at: Option<i64>,
    error_message: Option<String>,
}

impl FileRefreshStatusBuilder {
    pub fn status(mut self, value: CrawlStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn enqueued_at(mut self, value: i64) -> Self {
        self.enqueued_at = Some(value);
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

    pub fn last_synced_at(mut self, value: i64) -> Self {
        self.last_synced_at = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FileRefreshStatus`].
    pub fn build(self) -> Result<FileRefreshStatus, BuildError> {
        Ok(FileRefreshStatus {
            status: self.status,
            enqueued_at: self.enqueued_at,
            started_at: self.started_at,
            completed_at: self.completed_at,
            last_synced_at: self.last_synced_at,
            error_message: self.error_message,
        })
    }
}
