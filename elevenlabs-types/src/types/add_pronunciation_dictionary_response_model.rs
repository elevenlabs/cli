pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddPronunciationDictionaryResponseModel {
    /// The ID of the created pronunciation dictionary.
    #[serde(default)]
    pub id: String,
    /// The name of the created pronunciation dictionary.
    #[serde(default)]
    pub name: String,
    /// The user ID of the creator of the pronunciation dictionary.
    #[serde(default)]
    pub created_by: String,
    /// The creation time of the pronunciation dictionary in Unix timestamp.
    #[serde(default)]
    pub creation_time_unix: i64,
    /// The ID of the created pronunciation dictionary version.
    #[serde(default)]
    pub version_id: String,
    /// The number of rules in the version of the pronunciation dictionary.
    #[serde(default)]
    pub version_rules_num: i64,
    /// The description of the pronunciation dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The permission on the resource of the pronunciation dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_on_resource: Option<AddPronunciationDictionaryResponseModelPermissionOnResource>,
}

impl AddPronunciationDictionaryResponseModel {
    pub fn builder() -> AddPronunciationDictionaryResponseModelBuilder {
        <AddPronunciationDictionaryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddPronunciationDictionaryResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
    created_by: Option<String>,
    creation_time_unix: Option<i64>,
    version_id: Option<String>,
    version_rules_num: Option<i64>,
    description: Option<String>,
    permission_on_resource: Option<AddPronunciationDictionaryResponseModelPermissionOnResource>,
}

impl AddPronunciationDictionaryResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn version_rules_num(mut self, value: i64) -> Self {
        self.version_rules_num = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn permission_on_resource(mut self, value: AddPronunciationDictionaryResponseModelPermissionOnResource) -> Self {
        self.permission_on_resource = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddPronunciationDictionaryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AddPronunciationDictionaryResponseModelBuilder::id)
    /// - [`name`](AddPronunciationDictionaryResponseModelBuilder::name)
    /// - [`created_by`](AddPronunciationDictionaryResponseModelBuilder::created_by)
    /// - [`creation_time_unix`](AddPronunciationDictionaryResponseModelBuilder::creation_time_unix)
    /// - [`version_id`](AddPronunciationDictionaryResponseModelBuilder::version_id)
    /// - [`version_rules_num`](AddPronunciationDictionaryResponseModelBuilder::version_rules_num)
    pub fn build(self) -> Result<AddPronunciationDictionaryResponseModel, BuildError> {
        Ok(AddPronunciationDictionaryResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            created_by: self.created_by.ok_or_else(|| BuildError::missing_field("created_by"))?,
            creation_time_unix: self.creation_time_unix.ok_or_else(|| BuildError::missing_field("creation_time_unix"))?,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
            version_rules_num: self.version_rules_num.ok_or_else(|| BuildError::missing_field("version_rules_num"))?,
            description: self.description,
            permission_on_resource: self.permission_on_resource,
        })
    }
}
