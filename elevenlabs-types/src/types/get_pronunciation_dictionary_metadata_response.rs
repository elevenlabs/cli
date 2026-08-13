pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPronunciationDictionaryMetadataResponse {
    /// The ID of the pronunciation dictionary.
    #[serde(default)]
    pub id: String,
    /// The ID of the latest version of the pronunciation dictionary.
    #[serde(default)]
    pub latest_version_id: String,
    /// The number of rules in the latest version of the pronunciation dictionary.
    #[serde(default)]
    pub latest_version_rules_num: i64,
    /// The name of the pronunciation dictionary.
    #[serde(default)]
    pub name: String,
    /// The permission on the resource of the pronunciation dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_on_resource: Option<GetPronunciationDictionaryMetadataResponsePermissionOnResource>,
    /// The user ID of the creator of the pronunciation dictionary.
    #[serde(default)]
    pub created_by: String,
    /// The creation time of the pronunciation dictionary in Unix timestamp.
    #[serde(default)]
    pub creation_time_unix: i64,
    /// The archive time of the pronunciation dictionary in Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_time_unix: Option<i64>,
    /// The description of the pronunciation dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl GetPronunciationDictionaryMetadataResponse {
    pub fn builder() -> GetPronunciationDictionaryMetadataResponseBuilder {
        <GetPronunciationDictionaryMetadataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPronunciationDictionaryMetadataResponseBuilder {
    id: Option<String>,
    latest_version_id: Option<String>,
    latest_version_rules_num: Option<i64>,
    name: Option<String>,
    permission_on_resource: Option<GetPronunciationDictionaryMetadataResponsePermissionOnResource>,
    created_by: Option<String>,
    creation_time_unix: Option<i64>,
    archived_time_unix: Option<i64>,
    description: Option<String>,
}

impl GetPronunciationDictionaryMetadataResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn latest_version_id(mut self, value: impl Into<String>) -> Self {
        self.latest_version_id = Some(value.into());
        self
    }

    pub fn latest_version_rules_num(mut self, value: i64) -> Self {
        self.latest_version_rules_num = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn permission_on_resource(mut self, value: GetPronunciationDictionaryMetadataResponsePermissionOnResource) -> Self {
        self.permission_on_resource = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn creation_time_unix(mut self, value: i64) -> Self {
        self.creation_time_unix = Some(value);
        self
    }

    pub fn archived_time_unix(mut self, value: i64) -> Self {
        self.archived_time_unix = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetPronunciationDictionaryMetadataResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetPronunciationDictionaryMetadataResponseBuilder::id)
    /// - [`latest_version_id`](GetPronunciationDictionaryMetadataResponseBuilder::latest_version_id)
    /// - [`latest_version_rules_num`](GetPronunciationDictionaryMetadataResponseBuilder::latest_version_rules_num)
    /// - [`name`](GetPronunciationDictionaryMetadataResponseBuilder::name)
    /// - [`created_by`](GetPronunciationDictionaryMetadataResponseBuilder::created_by)
    /// - [`creation_time_unix`](GetPronunciationDictionaryMetadataResponseBuilder::creation_time_unix)
    pub fn build(self) -> Result<GetPronunciationDictionaryMetadataResponse, BuildError> {
        Ok(GetPronunciationDictionaryMetadataResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            latest_version_id: self.latest_version_id.ok_or_else(|| BuildError::missing_field("latest_version_id"))?,
            latest_version_rules_num: self.latest_version_rules_num.ok_or_else(|| BuildError::missing_field("latest_version_rules_num"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            permission_on_resource: self.permission_on_resource,
            created_by: self.created_by.ok_or_else(|| BuildError::missing_field("created_by"))?,
            creation_time_unix: self.creation_time_unix.ok_or_else(|| BuildError::missing_field("creation_time_unix"))?,
            archived_time_unix: self.archived_time_unix,
            description: self.description,
        })
    }
}
