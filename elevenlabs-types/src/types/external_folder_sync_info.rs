pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Metadata for a KB folder that mirrors an external source folder.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExternalFolderSyncInfo {
    /// Provider identifier
    pub r#type: ExternalSyncProvider,
    /// Entity ID in the external system
    #[serde(default)]
    pub source_entity_id: String,
    /// Integration connection instance ID
    #[serde(default)]
    pub integration_connection_id: String,
    /// KB folder ID of the sync root. None means this folder is the root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_folder_id: Option<String>,
    /// Opaque cursor for incremental sync, interpreted by the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_cursor: Option<String>,
    /// Unix timestamp of last completed sync
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_at: Option<i64>,
}

impl ExternalFolderSyncInfo {
    pub fn builder() -> ExternalFolderSyncInfoBuilder {
        <ExternalFolderSyncInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalFolderSyncInfoBuilder {
    r#type: Option<ExternalSyncProvider>,
    source_entity_id: Option<String>,
    integration_connection_id: Option<String>,
    root_folder_id: Option<String>,
    sync_cursor: Option<String>,
    last_sync_at: Option<i64>,
}

impl ExternalFolderSyncInfoBuilder {
    pub fn r#type(mut self, value: ExternalSyncProvider) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn source_entity_id(mut self, value: impl Into<String>) -> Self {
        self.source_entity_id = Some(value.into());
        self
    }

    pub fn integration_connection_id(mut self, value: impl Into<String>) -> Self {
        self.integration_connection_id = Some(value.into());
        self
    }

    pub fn root_folder_id(mut self, value: impl Into<String>) -> Self {
        self.root_folder_id = Some(value.into());
        self
    }

    pub fn sync_cursor(mut self, value: impl Into<String>) -> Self {
        self.sync_cursor = Some(value.into());
        self
    }

    pub fn last_sync_at(mut self, value: i64) -> Self {
        self.last_sync_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExternalFolderSyncInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ExternalFolderSyncInfoBuilder::r#type)
    /// - [`source_entity_id`](ExternalFolderSyncInfoBuilder::source_entity_id)
    /// - [`integration_connection_id`](ExternalFolderSyncInfoBuilder::integration_connection_id)
    pub fn build(self) -> Result<ExternalFolderSyncInfo, BuildError> {
        Ok(ExternalFolderSyncInfo {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            source_entity_id: self.source_entity_id.ok_or_else(|| BuildError::missing_field("source_entity_id"))?,
            integration_connection_id: self.integration_connection_id.ok_or_else(|| BuildError::missing_field("integration_connection_id"))?,
            root_folder_id: self.root_folder_id,
            sync_cursor: self.sync_cursor,
            last_sync_at: self.last_sync_at,
        })
    }
}
