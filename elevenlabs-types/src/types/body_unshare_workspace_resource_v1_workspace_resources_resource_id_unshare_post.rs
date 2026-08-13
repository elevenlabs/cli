pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePost {
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

impl BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePost {
    pub fn builder() -> BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePostBuilder {
        <BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePostBuilder {
    resource_type: Option<WorkspaceResourceType>,
    user_email: Option<String>,
    group_id: Option<String>,
    workspace_api_key_id: Option<String>,
}

impl BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePostBuilder {
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

    /// Consumes the builder and constructs a [`BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource_type`](BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePostBuilder::resource_type)
    pub fn build(self) -> Result<BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePost, BuildError> {
        Ok(BodyUnshareWorkspaceResourceV1WorkspaceResourcesResourceIdUnsharePost {
            resource_type: self.resource_type.ok_or_else(|| BuildError::missing_field("resource_type"))?,
            user_email: self.user_email,
            group_id: self.group_id,
            workspace_api_key_id: self.workspace_api_key_id,
        })
    }
}

