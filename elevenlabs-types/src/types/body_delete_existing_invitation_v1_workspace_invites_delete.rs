pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyDeleteExistingInvitationV1WorkspaceInvitesDelete {
    /// The email of the customer
    #[serde(default)]
    pub email: String,
}

impl BodyDeleteExistingInvitationV1WorkspaceInvitesDelete {
    pub fn builder() -> BodyDeleteExistingInvitationV1WorkspaceInvitesDeleteBuilder {
        <BodyDeleteExistingInvitationV1WorkspaceInvitesDeleteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyDeleteExistingInvitationV1WorkspaceInvitesDeleteBuilder {
    email: Option<String>,
}

impl BodyDeleteExistingInvitationV1WorkspaceInvitesDeleteBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyDeleteExistingInvitationV1WorkspaceInvitesDelete`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](BodyDeleteExistingInvitationV1WorkspaceInvitesDeleteBuilder::email)
    pub fn build(self) -> Result<BodyDeleteExistingInvitationV1WorkspaceInvitesDelete, BuildError> {
        Ok(BodyDeleteExistingInvitationV1WorkspaceInvitesDelete {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
        })
    }
}

