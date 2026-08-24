pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePost {
    /// The email of the target workspace member.
    #[serde(default)]
    pub email: String,
}

impl BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePost {
    pub fn builder() -> BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePostBuilder {
        <BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePostBuilder {
    email: Option<String>,
}

impl BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePostBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePostBuilder::email)
    pub fn build(self) -> Result<BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePost, BuildError> {
        Ok(BodyDeleteMemberFromUserGroupV1WorkspaceGroupsGroupIdMembersRemovePost {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
        })
    }
}

