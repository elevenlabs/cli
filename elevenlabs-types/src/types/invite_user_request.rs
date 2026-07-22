pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InviteUserRequest {
    /// The email of the customer
    #[serde(default)]
    pub email: String,
    /// The workspace permission of the user. This is deprecated, use `seat_type` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_permission: Option<String>,
    /// The seat type of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_type: Option<SeatType>,
    /// The group ids of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Monthly credit usage limit for the invitee. Omit or set to null for no custom cap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_limit: Option<i64>,
}

impl InviteUserRequest {
    pub fn builder() -> InviteUserRequestBuilder {
        <InviteUserRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteUserRequestBuilder {
    email: Option<String>,
    workspace_permission: Option<String>,
    seat_type: Option<SeatType>,
    group_ids: Option<Vec<String>>,
    usage_limit: Option<i64>,
}

impl InviteUserRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn workspace_permission(mut self, value: impl Into<String>) -> Self {
        self.workspace_permission = Some(value.into());
        self
    }

    pub fn seat_type(mut self, value: SeatType) -> Self {
        self.seat_type = Some(value);
        self
    }

    pub fn group_ids(mut self, value: Vec<String>) -> Self {
        self.group_ids = Some(value);
        self
    }

    pub fn usage_limit(mut self, value: i64) -> Self {
        self.usage_limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InviteUserRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](InviteUserRequestBuilder::email)
    pub fn build(self) -> Result<InviteUserRequest, BuildError> {
        Ok(InviteUserRequest {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            workspace_permission: self.workspace_permission,
            seat_type: self.seat_type,
            group_ids: self.group_ids,
            usage_limit: self.usage_limit,
        })
    }
}

