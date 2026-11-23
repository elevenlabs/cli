pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamInviteRequest {
    /// The email address to invite.
    #[serde(default)]
    pub email: String,
    /// The role the invitee joins with: 'workspace_admin' or 'workspace_member'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<TeamInviteRequestRole>,
}

impl TeamInviteRequest {
    pub fn builder() -> TeamInviteRequestBuilder {
        <TeamInviteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamInviteRequestBuilder {
    email: Option<String>,
    role: Option<TeamInviteRequestRole>,
}

impl TeamInviteRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role(mut self, value: TeamInviteRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamInviteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](TeamInviteRequestBuilder::email)
    pub fn build(self) -> Result<TeamInviteRequest, BuildError> {
        Ok(TeamInviteRequest {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            role: self.role,
        })
    }
}
