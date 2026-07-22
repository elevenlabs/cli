pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceMetadataResponseModel {
    /// The ID of the resource.
    #[serde(default)]
    pub resource_id: String,
    /// The name of the resource, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// The type of the resource.
    pub resource_type: WorkspaceResourceType,
    /// The ID of the user who created the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    /// The access level for anonymous users. If None, the resource is not shared publicly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_access_level_override: Option<ResourceMetadataResponseModelAnonymousAccessLevelOverride>,
    /// A mapping of roles to group IDs. When the resource is shared with a user, the group id is the user's id.
    #[serde(default)]
    pub role_to_group_ids: HashMap<String, Vec<String>>,
    /// List of options for sharing the resource further in the workspace. These are users who don't have access to the resource yet.
    #[serde(default)]
    pub share_options: Vec<ShareOptionResponseModel>,
}

impl ResourceMetadataResponseModel {
    pub fn builder() -> ResourceMetadataResponseModelBuilder {
        <ResourceMetadataResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResourceMetadataResponseModelBuilder {
    resource_id: Option<String>,
    resource_name: Option<String>,
    resource_type: Option<WorkspaceResourceType>,
    creator_user_id: Option<String>,
    anonymous_access_level_override: Option<ResourceMetadataResponseModelAnonymousAccessLevelOverride>,
    role_to_group_ids: Option<HashMap<String, Vec<String>>>,
    share_options: Option<Vec<ShareOptionResponseModel>>,
}

impl ResourceMetadataResponseModelBuilder {
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn resource_name(mut self, value: impl Into<String>) -> Self {
        self.resource_name = Some(value.into());
        self
    }

    pub fn resource_type(mut self, value: WorkspaceResourceType) -> Self {
        self.resource_type = Some(value);
        self
    }

    pub fn creator_user_id(mut self, value: impl Into<String>) -> Self {
        self.creator_user_id = Some(value.into());
        self
    }

    pub fn anonymous_access_level_override(mut self, value: ResourceMetadataResponseModelAnonymousAccessLevelOverride) -> Self {
        self.anonymous_access_level_override = Some(value);
        self
    }

    pub fn role_to_group_ids(mut self, value: HashMap<String, Vec<String>>) -> Self {
        self.role_to_group_ids = Some(value);
        self
    }

    pub fn share_options(mut self, value: Vec<ShareOptionResponseModel>) -> Self {
        self.share_options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResourceMetadataResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource_id`](ResourceMetadataResponseModelBuilder::resource_id)
    /// - [`resource_type`](ResourceMetadataResponseModelBuilder::resource_type)
    /// - [`role_to_group_ids`](ResourceMetadataResponseModelBuilder::role_to_group_ids)
    /// - [`share_options`](ResourceMetadataResponseModelBuilder::share_options)
    pub fn build(self) -> Result<ResourceMetadataResponseModel, BuildError> {
        Ok(ResourceMetadataResponseModel {
            resource_id: self.resource_id.ok_or_else(|| BuildError::missing_field("resource_id"))?,
            resource_name: self.resource_name,
            resource_type: self.resource_type.ok_or_else(|| BuildError::missing_field("resource_type"))?,
            creator_user_id: self.creator_user_id,
            anonymous_access_level_override: self.anonymous_access_level_override,
            role_to_group_ids: self.role_to_group_ids.ok_or_else(|| BuildError::missing_field("role_to_group_ids"))?,
            share_options: self.share_options.ok_or_else(|| BuildError::missing_field("share_options"))?,
        })
    }
}
