pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMemberRequest {
    /// Email of the target user.
    #[serde(default)]
    pub email: String,
    /// Whether to lock or unlock the user account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    /// The workspace role of the user. This is deprecated, use `workspace_seat_type` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_role: Option<SeatType>,
    /// The workspace seat type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_seat_type: Option<SeatType>,
}

impl UpdateMemberRequest {
    pub fn builder() -> UpdateMemberRequestBuilder {
        <UpdateMemberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMemberRequestBuilder {
    email: Option<String>,
    is_locked: Option<bool>,
    workspace_role: Option<SeatType>,
    workspace_seat_type: Option<SeatType>,
}

impl UpdateMemberRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn is_locked(mut self, value: bool) -> Self {
        self.is_locked = Some(value);
        self
    }

    pub fn workspace_role(mut self, value: SeatType) -> Self {
        self.workspace_role = Some(value);
        self
    }

    pub fn workspace_seat_type(mut self, value: SeatType) -> Self {
        self.workspace_seat_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMemberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](UpdateMemberRequestBuilder::email)
    pub fn build(self) -> Result<UpdateMemberRequest, BuildError> {
        Ok(UpdateMemberRequest {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            is_locked: self.is_locked,
            workspace_role: self.workspace_role,
            workspace_seat_type: self.workspace_seat_type,
        })
    }
}

