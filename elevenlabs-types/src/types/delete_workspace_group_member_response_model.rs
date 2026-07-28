pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteWorkspaceGroupMemberResponseModel {
    /// The status of the workspace group member deletion request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl DeleteWorkspaceGroupMemberResponseModel {
    pub fn builder() -> DeleteWorkspaceGroupMemberResponseModelBuilder {
        <DeleteWorkspaceGroupMemberResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteWorkspaceGroupMemberResponseModelBuilder {
    status: Option<String>,
}

impl DeleteWorkspaceGroupMemberResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteWorkspaceGroupMemberResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DeleteWorkspaceGroupMemberResponseModelBuilder::status)
    pub fn build(self) -> Result<DeleteWorkspaceGroupMemberResponseModel, BuildError> {
        Ok(DeleteWorkspaceGroupMemberResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
