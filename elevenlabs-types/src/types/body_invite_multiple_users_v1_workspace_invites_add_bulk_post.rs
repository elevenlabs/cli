pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyInviteMultipleUsersV1WorkspaceInvitesAddBulkPost {
    /// The email of the customer
    #[serde(default)]
    pub emails: Vec<String>,
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

impl BodyInviteMultipleUsersV1WorkspaceInvitesAddBulkPost {
    pub fn builder() -> BodyInviteMultipleUsersV1WorkspaceInvitesAddBulkPostBuilder {
        <BodyInviteMultipleUsersV1WorkspaceInvitesAddBulkPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyInviteMultipleUsersV1WorkspaceInvitesAddBulkPostBuilder {
    emails: Option<Vec<String>>,
    seat_type: Option<SeatType>,
    group_ids: Option<Vec<String>>,
    usage_limit: Option<i64>,
}

impl BodyInviteMultipleUsersV1WorkspaceInvitesAddBulkPostBuilder {
    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
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

    /// Consumes the builder and constructs a [`BodyInviteMultipleUsersV1WorkspaceInvitesAddBulkPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`emails`](BodyInviteMultipleUsersV1WorkspaceInvitesAddBulkPostBuilder::emails)
    pub fn build(self) -> Result<BodyInviteMultipleUsersV1WorkspaceInvitesAddBulkPost, BuildError> {
        Ok(BodyInviteMultipleUsersV1WorkspaceInvitesAddBulkPost {
            emails: self.emails.ok_or_else(|| BuildError::missing_field("emails"))?,
            seat_type: self.seat_type,
            group_ids: self.group_ids,
            usage_limit: self.usage_limit,
        })
    }
}

