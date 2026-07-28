pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public workspace member fields exposed via GET /v1/workspace/members.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceMemberResponseModel {
    /// The user ID of the workspace member.
    #[serde(default)]
    pub user_id: String,
    /// The email address of the workspace member.
    #[serde(default)]
    pub email: String,
    /// The first name of the workspace member, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The seat type (role) of the workspace member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_type: Option<SeatType>,
    /// Whether the member is the workspace owner.
    #[serde(default)]
    pub is_owner: bool,
    /// Whether the member's account is locked in this workspace.
    #[serde(default)]
    pub is_locked: bool,
}

impl WorkspaceMemberResponseModel {
    pub fn builder() -> WorkspaceMemberResponseModelBuilder {
        <WorkspaceMemberResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceMemberResponseModelBuilder {
    user_id: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    seat_type: Option<SeatType>,
    is_owner: Option<bool>,
    is_locked: Option<bool>,
}

impl WorkspaceMemberResponseModelBuilder {
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

    pub fn seat_type(mut self, value: SeatType) -> Self {
        self.seat_type = Some(value);
        self
    }

    pub fn is_owner(mut self, value: bool) -> Self {
        self.is_owner = Some(value);
        self
    }

    pub fn is_locked(mut self, value: bool) -> Self {
        self.is_locked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceMemberResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_id`](WorkspaceMemberResponseModelBuilder::user_id)
    /// - [`email`](WorkspaceMemberResponseModelBuilder::email)
    /// - [`is_owner`](WorkspaceMemberResponseModelBuilder::is_owner)
    /// - [`is_locked`](WorkspaceMemberResponseModelBuilder::is_locked)
    pub fn build(self) -> Result<WorkspaceMemberResponseModel, BuildError> {
        Ok(WorkspaceMemberResponseModel {
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self.first_name,
            seat_type: self.seat_type,
            is_owner: self.is_owner.ok_or_else(|| BuildError::missing_field("is_owner"))?,
            is_locked: self.is_locked.ok_or_else(|| BuildError::missing_field("is_locked"))?,
        })
    }
}
