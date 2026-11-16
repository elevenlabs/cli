pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTeamInviteRequest {
    /// The email address to invite.
    #[serde(default)]
    pub email: String,
    /// The role the invitee joins with: 'workspace_admin' or 'workspace_member'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<CreateTeamInviteRequestRole>,
}

impl CreateTeamInviteRequest {
    pub fn builder() -> CreateTeamInviteRequestBuilder {
        <CreateTeamInviteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTeamInviteRequestBuilder {
    email: Option<String>,
    role: Option<CreateTeamInviteRequestRole>,
}

impl CreateTeamInviteRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role(mut self, value: CreateTeamInviteRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTeamInviteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](CreateTeamInviteRequestBuilder::email)
    pub fn build(self) -> Result<CreateTeamInviteRequest, BuildError> {
        Ok(CreateTeamInviteRequest {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            role: self.role,
        })
    }
}

