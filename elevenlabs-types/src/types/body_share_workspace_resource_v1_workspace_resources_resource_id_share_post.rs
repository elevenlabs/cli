pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePost {
    /// Role to grant to the target: one of 'admin', 'editor', 'commenter', or 'viewer'.
    pub role: BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePostRole,
    /// Resource type of the target resource.
    pub resource_type: WorkspaceResourceType,
    /// The email of the user or service account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    /// The ID of the target group. Use 'default' to set the resource's baseline role — every workspace member receives this role unless they hold a higher one through a direct user grant, group membership, or workspace (service account) API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The ID of the target workspace (service account) API key. This is not the API key string itself that you pass in the header for authentication — it is the key's ID, which workspace admins can find under Developers → Service Accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_api_key_id: Option<String>,
}

impl BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePost {
    pub fn builder() -> BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePostBuilder {
        <BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePostBuilder {
    role: Option<BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePostRole>,
    resource_type: Option<WorkspaceResourceType>,
    user_email: Option<String>,
    group_id: Option<String>,
    workspace_api_key_id: Option<String>,
}

impl BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePostBuilder {
    pub fn role(mut self, value: BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePostRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn resource_type(mut self, value: WorkspaceResourceType) -> Self {
        self.resource_type = Some(value);
        self
    }

    pub fn user_email(mut self, value: impl Into<String>) -> Self {
        self.user_email = Some(value.into());
        self
    }

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn workspace_api_key_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_api_key_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePostBuilder::role)
    /// - [`resource_type`](BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePostBuilder::resource_type)
    pub fn build(self) -> Result<BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePost, BuildError> {
        Ok(BodyShareWorkspaceResourceV1WorkspaceResourcesResourceIdSharePost {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            resource_type: self.resource_type.ok_or_else(|| BuildError::missing_field("resource_type"))?,
            user_email: self.user_email,
            group_id: self.group_id,
            workspace_api_key_id: self.workspace_api_key_id,
        })
    }
}

