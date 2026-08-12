pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteWorkspaceInviteResponseModel {
    /// The status of the workspace invite deletion request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl DeleteWorkspaceInviteResponseModel {
    pub fn builder() -> DeleteWorkspaceInviteResponseModelBuilder {
        <DeleteWorkspaceInviteResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteWorkspaceInviteResponseModelBuilder {
    status: Option<String>,
}

impl DeleteWorkspaceInviteResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteWorkspaceInviteResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DeleteWorkspaceInviteResponseModelBuilder::status)
    pub fn build(self) -> Result<DeleteWorkspaceInviteResponseModel, BuildError> {
        Ok(DeleteWorkspaceInviteResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
