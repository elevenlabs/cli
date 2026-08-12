pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddWorkspaceInviteResponseModel {
    /// The status of the workspace invite request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl AddWorkspaceInviteResponseModel {
    pub fn builder() -> AddWorkspaceInviteResponseModelBuilder {
        <AddWorkspaceInviteResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddWorkspaceInviteResponseModelBuilder {
    status: Option<String>,
}

impl AddWorkspaceInviteResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddWorkspaceInviteResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](AddWorkspaceInviteResponseModelBuilder::status)
    pub fn build(self) -> Result<AddWorkspaceInviteResponseModel, BuildError> {
        Ok(AddWorkspaceInviteResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
