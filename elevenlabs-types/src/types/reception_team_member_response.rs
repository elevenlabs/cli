pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionTeamMemberResponse {
    /// The member's user ID.
    #[serde(default)]
    pub user_id: String,
    /// The member's email address.
    #[serde(default)]
    pub email: String,
    /// The member's first name, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Whether the member owns the workspace.
    #[serde(default)]
    pub is_owner: bool,
    /// Whether the member is a workspace admin.
    #[serde(default)]
    pub is_admin: bool,
    /// URL of the member's profile picture, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    /// Whether the member's access to the workspace is suspended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_suspended: Option<bool>,
}

impl ReceptionTeamMemberResponse {
    pub fn builder() -> ReceptionTeamMemberResponseBuilder {
        <ReceptionTeamMemberResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionTeamMemberResponseBuilder {
    user_id: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    is_owner: Option<bool>,
    is_admin: Option<bool>,
    picture_url: Option<String>,
    is_suspended: Option<bool>,
}

impl ReceptionTeamMemberResponseBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn is_owner(mut self, value: bool) -> Self {
        self.is_owner = Some(value);
        self
    }

    pub fn is_admin(mut self, value: bool) -> Self {
        self.is_admin = Some(value);
        self
    }

    pub fn picture_url(mut self, value: impl Into<String>) -> Self {
        self.picture_url = Some(value.into());
        self
    }

    pub fn is_suspended(mut self, value: bool) -> Self {
        self.is_suspended = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionTeamMemberResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](ReceptionTeamMemberResponseBuilder::user_id)
    /// - [`email`](ReceptionTeamMemberResponseBuilder::email)
    /// - [`is_owner`](ReceptionTeamMemberResponseBuilder::is_owner)
    /// - [`is_admin`](ReceptionTeamMemberResponseBuilder::is_admin)
    pub fn build(self) -> Result<ReceptionTeamMemberResponse, BuildError> {
        Ok(ReceptionTeamMemberResponse {
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self.first_name,
            is_owner: self.is_owner.ok_or_else(|| BuildError::missing_field("is_owner"))?,
            is_admin: self.is_admin.ok_or_else(|| BuildError::missing_field("is_admin"))?,
            picture_url: self.picture_url,
            is_suspended: self.is_suspended,
        })
    }
}
