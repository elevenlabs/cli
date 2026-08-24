pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Tracks the link back to the original file in an external source.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExternalFileSyncInfo {
    /// Provider identifier
    pub r#type: ExternalSyncProvider,
    /// Entity ID in the external system
    #[serde(default)]
    pub source_entity_id: String,
    /// Integration connection instance ID
    #[serde(default)]
    pub integration_connection_id: String,
    /// Folder ID in the external system this file was synced from
    #[serde(default)]
    pub source_parent_entity_id: String,
    /// Original MIME type in the external system
    #[serde(default)]
    pub source_mime_type: String,
    /// Last modified time from the external system
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub source_modified_time: DateTime<FixedOffset>,
    /// KB folder ID of the sync root, used to query all entities under a sync tree
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_folder_id: Option<String>,
}

impl ExternalFileSyncInfo {
    pub fn builder() -> ExternalFileSyncInfoBuilder {
        <ExternalFileSyncInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalFileSyncInfoBuilder {
    r#type: Option<ExternalSyncProvider>,
    source_entity_id: Option<String>,
    integration_connection_id: Option<String>,
    source_parent_entity_id: Option<String>,
    source_mime_type: Option<String>,
    source_modified_time: Option<DateTime<FixedOffset>>,
    root_folder_id: Option<String>,
}

impl ExternalFileSyncInfoBuilder {
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

    pub fn source_parent_entity_id(mut self, value: impl Into<String>) -> Self {
        self.source_parent_entity_id = Some(value.into());
        self
    }

    pub fn source_mime_type(mut self, value: impl Into<String>) -> Self {
        self.source_mime_type = Some(value.into());
        self
    }

    pub fn source_modified_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.source_modified_time = Some(value);
        self
    }

    pub fn root_folder_id(mut self, value: impl Into<String>) -> Self {
        self.root_folder_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExternalFileSyncInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ExternalFileSyncInfoBuilder::r#type)
    /// - [`source_entity_id`](ExternalFileSyncInfoBuilder::source_entity_id)
    /// - [`integration_connection_id`](ExternalFileSyncInfoBuilder::integration_connection_id)
    /// - [`source_parent_entity_id`](ExternalFileSyncInfoBuilder::source_parent_entity_id)
    /// - [`source_mime_type`](ExternalFileSyncInfoBuilder::source_mime_type)
    /// - [`source_modified_time`](ExternalFileSyncInfoBuilder::source_modified_time)
    pub fn build(self) -> Result<ExternalFileSyncInfo, BuildError> {
        Ok(ExternalFileSyncInfo {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            source_entity_id: self.source_entity_id.ok_or_else(|| BuildError::missing_field("source_entity_id"))?,
            integration_connection_id: self.integration_connection_id.ok_or_else(|| BuildError::missing_field("integration_connection_id"))?,
            source_parent_entity_id: self.source_parent_entity_id.ok_or_else(|| BuildError::missing_field("source_parent_entity_id"))?,
            source_mime_type: self.source_mime_type.ok_or_else(|| BuildError::missing_field("source_mime_type"))?,
            source_modified_time: self.source_modified_time.ok_or_else(|| BuildError::missing_field("source_modified_time"))?,
            root_folder_id: self.root_folder_id,
        })
    }
}
