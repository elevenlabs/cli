pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DefaultSharingGroupResponseModel {
    /// The group to share with by default
    #[serde(default)]
    pub group: WorkspaceGroupResponseModel,
    /// The permission level to grant to the group
    pub permission_level: DefaultSharingGroupResponseModelPermissionLevel,
}

impl DefaultSharingGroupResponseModel {
    pub fn builder() -> DefaultSharingGroupResponseModelBuilder {
        <DefaultSharingGroupResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DefaultSharingGroupResponseModelBuilder {
    group: Option<WorkspaceGroupResponseModel>,
    permission_level: Option<DefaultSharingGroupResponseModelPermissionLevel>,
}

impl DefaultSharingGroupResponseModelBuilder {
    pub fn group(mut self, value: WorkspaceGroupResponseModel) -> Self {
        self.group = Some(value);
        self
    }

    pub fn permission_level(mut self, value: DefaultSharingGroupResponseModelPermissionLevel) -> Self {
        self.permission_level = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DefaultSharingGroupResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group`](DefaultSharingGroupResponseModelBuilder::group)
    /// - [`permission_level`](DefaultSharingGroupResponseModelBuilder::permission_level)
    pub fn build(self) -> Result<DefaultSharingGroupResponseModel, BuildError> {
        Ok(DefaultSharingGroupResponseModel {
            group: self.group.ok_or_else(|| BuildError::missing_field("group"))?,
            permission_level: self.permission_level.ok_or_else(|| BuildError::missing_field("permission_level"))?,
        })
    }
}
