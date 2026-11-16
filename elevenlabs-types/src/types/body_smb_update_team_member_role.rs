pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodySmbUpdateTeamMemberRole {
    /// The member's new role.
    pub role: BodySmbUpdateTeamMemberRoleRole,
}

impl BodySmbUpdateTeamMemberRole {
    pub fn builder() -> BodySmbUpdateTeamMemberRoleBuilder {
        <BodySmbUpdateTeamMemberRoleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodySmbUpdateTeamMemberRoleBuilder {
    role: Option<BodySmbUpdateTeamMemberRoleRole>,
}

impl BodySmbUpdateTeamMemberRoleBuilder {
    pub fn role(mut self, value: BodySmbUpdateTeamMemberRoleRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodySmbUpdateTeamMemberRole`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](BodySmbUpdateTeamMemberRoleBuilder::role)
    pub fn build(self) -> Result<BodySmbUpdateTeamMemberRole, BuildError> {
        Ok(BodySmbUpdateTeamMemberRole {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}

