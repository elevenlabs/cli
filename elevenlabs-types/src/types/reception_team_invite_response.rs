pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReceptionTeamInviteResponse {
    /// The invited email address.
    #[serde(default)]
    pub email: String,
    /// The role the invitee joins with.
    pub seat_type: SeatType,
    /// Email address of the member who sent the invite, if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviting_user_email: Option<String>,
    /// When the invite was sent, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<i64>,
}

impl ReceptionTeamInviteResponse {
    pub fn builder() -> ReceptionTeamInviteResponseBuilder {
        <ReceptionTeamInviteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionTeamInviteResponseBuilder {
    email: Option<String>,
    seat_type: Option<SeatType>,
    inviting_user_email: Option<String>,
    created_at_unix: Option<i64>,
}

impl ReceptionTeamInviteResponseBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn seat_type(mut self, value: SeatType) -> Self {
        self.seat_type = Some(value);
        self
    }

    pub fn inviting_user_email(mut self, value: impl Into<String>) -> Self {
        self.inviting_user_email = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionTeamInviteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](ReceptionTeamInviteResponseBuilder::email)
    /// - [`seat_type`](ReceptionTeamInviteResponseBuilder::seat_type)
    pub fn build(self) -> Result<ReceptionTeamInviteResponse, BuildError> {
        Ok(ReceptionTeamInviteResponse {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            seat_type: self.seat_type.ok_or_else(|| BuildError::missing_field("seat_type"))?,
            inviting_user_email: self.inviting_user_email,
            created_at_unix: self.created_at_unix,
        })
    }
}
