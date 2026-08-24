pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PronunciationDictionaryVersionResponseModel {
    #[serde(default)]
    pub version_id: String,
    #[serde(default)]
    pub version_rules_num: i64,
    #[serde(default)]
    pub pronunciation_dictionary_id: String,
    #[serde(default)]
    pub dictionary_name: String,
    #[serde(default)]
    pub version_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_on_resource: Option<PronunciationDictionaryVersionResponseModelPermissionOnResource>,
    #[serde(default)]
    pub created_by: String,
    #[serde(default)]
    pub creation_time_unix: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_time_unix: Option<i64>,
}

impl PronunciationDictionaryVersionResponseModel {
    pub fn builder() -> PronunciationDictionaryVersionResponseModelBuilder {
        <PronunciationDictionaryVersionResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionaryVersionResponseModelBuilder {
    version_id: Option<String>,
    version_rules_num: Option<i64>,
    pronunciation_dictionary_id: Option<String>,
    dictionary_name: Option<String>,
    version_name: Option<String>,
    permission_on_resource: Option<PronunciationDictionaryVersionResponseModelPermissionOnResource>,
    created_by: Option<String>,
    creation_time_unix: Option<i64>,
    archived_time_unix: Option<i64>,
}

impl PronunciationDictionaryVersionResponseModelBuilder {
    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn version_rules_num(mut self, value: i64) -> Self {
        self.version_rules_num = Some(value);
        self
    }

    pub fn pronunciation_dictionary_id(mut self, value: impl Into<String>) -> Self {
        self.pronunciation_dictionary_id = Some(value.into());
        self
    }

    pub fn dictionary_name(mut self, value: impl Into<String>) -> Self {
        self.dictionary_name = Some(value.into());
        self
    }

    pub fn version_name(mut self, value: impl Into<String>) -> Self {
        self.version_name = Some(value.into());
        self
    }

    pub fn permission_on_resource(mut self, value: PronunciationDictionaryVersionResponseModelPermissionOnResource) -> Self {
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

    /// Consumes the builder and constructs a [`PronunciationDictionaryVersionResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version_id`](PronunciationDictionaryVersionResponseModelBuilder::version_id)
    /// - [`version_rules_num`](PronunciationDictionaryVersionResponseModelBuilder::version_rules_num)
    /// - [`pronunciation_dictionary_id`](PronunciationDictionaryVersionResponseModelBuilder::pronunciation_dictionary_id)
    /// - [`dictionary_name`](PronunciationDictionaryVersionResponseModelBuilder::dictionary_name)
    /// - [`version_name`](PronunciationDictionaryVersionResponseModelBuilder::version_name)
    /// - [`created_by`](PronunciationDictionaryVersionResponseModelBuilder::created_by)
    /// - [`creation_time_unix`](PronunciationDictionaryVersionResponseModelBuilder::creation_time_unix)
    pub fn build(self) -> Result<PronunciationDictionaryVersionResponseModel, BuildError> {
        Ok(PronunciationDictionaryVersionResponseModel {
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
            version_rules_num: self.version_rules_num.ok_or_else(|| BuildError::missing_field("version_rules_num"))?,
            pronunciation_dictionary_id: self.pronunciation_dictionary_id.ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_id"))?,
            dictionary_name: self.dictionary_name.ok_or_else(|| BuildError::missing_field("dictionary_name"))?,
            version_name: self.version_name.ok_or_else(|| BuildError::missing_field("version_name"))?,
            permission_on_resource: self.permission_on_resource,
            created_by: self.created_by.ok_or_else(|| BuildError::missing_field("created_by"))?,
            creation_time_unix: self.creation_time_unix.ok_or_else(|| BuildError::missing_field("creation_time_unix"))?,
            archived_time_unix: self.archived_time_unix,
        })
    }
}
