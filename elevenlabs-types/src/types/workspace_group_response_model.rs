pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceGroupResponseModel {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<WorkspaceGroupPermission>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_usage_limit: Option<WorkspaceGroupResponseModelGroupUsageLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_pvc_limit: Option<WorkspaceGroupResponseModelGroupPvcLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_scim_synced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scim_group: Option<ScimGroupResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scim_frozen: Option<bool>,
}

impl WorkspaceGroupResponseModel {
    pub fn builder() -> WorkspaceGroupResponseModelBuilder {
        <WorkspaceGroupResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceGroupResponseModelBuilder {
    name: Option<String>,
    id: Option<String>,
    members: Option<Vec<String>>,
    permissions: Option<Vec<WorkspaceGroupPermission>>,
    group_usage_limit: Option<WorkspaceGroupResponseModelGroupUsageLimit>,
    group_pvc_limit: Option<WorkspaceGroupResponseModelGroupPvcLimit>,
    character_count: Option<i64>,
    is_scim_synced: Option<bool>,
    scim_group: Option<ScimGroupResponseModel>,
    scim_frozen: Option<bool>,
}

impl WorkspaceGroupResponseModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn members(mut self, value: Vec<String>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn permissions(mut self, value: Vec<WorkspaceGroupPermission>) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn group_usage_limit(mut self, value: WorkspaceGroupResponseModelGroupUsageLimit) -> Self {
        self.group_usage_limit = Some(value);
        self
    }

    pub fn group_pvc_limit(mut self, value: WorkspaceGroupResponseModelGroupPvcLimit) -> Self {
        self.group_pvc_limit = Some(value);
        self
    }

    pub fn character_count(mut self, value: i64) -> Self {
        self.character_count = Some(value);
        self
    }

    pub fn is_scim_synced(mut self, value: bool) -> Self {
        self.is_scim_synced = Some(value);
        self
    }

    pub fn scim_group(mut self, value: ScimGroupResponseModel) -> Self {
        self.scim_group = Some(value);
        self
    }

    pub fn scim_frozen(mut self, value: bool) -> Self {
        self.scim_frozen = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceGroupResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](WorkspaceGroupResponseModelBuilder::name)
    /// - [`id`](WorkspaceGroupResponseModelBuilder::id)
    /// - [`members`](WorkspaceGroupResponseModelBuilder::members)
    pub fn build(self) -> Result<WorkspaceGroupResponseModel, BuildError> {
        Ok(WorkspaceGroupResponseModel {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            members: self.members.ok_or_else(|| BuildError::missing_field("members"))?,
            permissions: self.permissions,
            group_usage_limit: self.group_usage_limit,
            group_pvc_limit: self.group_pvc_limit,
            character_count: self.character_count,
            is_scim_synced: self.is_scim_synced,
            scim_group: self.scim_group,
            scim_frozen: self.scim_frozen,
        })
    }
}
