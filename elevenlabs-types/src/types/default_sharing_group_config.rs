pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DefaultSharingGroupConfig {
    /// The ID of the group to share with
    #[serde(default)]
    pub group_id: String,
    /// The permission level to grant to the group
    pub permission_level: DefaultSharingGroupConfigPermissionLevel,
}

impl DefaultSharingGroupConfig {
    pub fn builder() -> DefaultSharingGroupConfigBuilder {
        <DefaultSharingGroupConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DefaultSharingGroupConfigBuilder {
    group_id: Option<String>,
    permission_level: Option<DefaultSharingGroupConfigPermissionLevel>,
}

impl DefaultSharingGroupConfigBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn permission_level(mut self, value: DefaultSharingGroupConfigPermissionLevel) -> Self {
        self.permission_level = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DefaultSharingGroupConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](DefaultSharingGroupConfigBuilder::group_id)
    /// - [`permission_level`](DefaultSharingGroupConfigBuilder::permission_level)
    pub fn build(self) -> Result<DefaultSharingGroupConfig, BuildError> {
        Ok(DefaultSharingGroupConfig {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            permission_level: self.permission_level.ok_or_else(|| BuildError::missing_field("permission_level"))?,
        })
    }
}
