pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodySmbCancelTeamInvite {
    /// Email address the pending invite was sent to.
    #[serde(default)]
    pub email: String,
}

impl BodySmbCancelTeamInvite {
    pub fn builder() -> BodySmbCancelTeamInviteBuilder {
        <BodySmbCancelTeamInviteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodySmbCancelTeamInviteBuilder {
    email: Option<String>,
}

impl BodySmbCancelTeamInviteBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodySmbCancelTeamInvite`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](BodySmbCancelTeamInviteBuilder::email)
    pub fn build(self) -> Result<BodySmbCancelTeamInvite, BuildError> {
        Ok(BodySmbCancelTeamInvite {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
        })
    }
}

