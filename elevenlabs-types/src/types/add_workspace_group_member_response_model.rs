pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddWorkspaceGroupMemberResponseModel {
    /// The status of the workspace group member addition request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl AddWorkspaceGroupMemberResponseModel {
    pub fn builder() -> AddWorkspaceGroupMemberResponseModelBuilder {
        <AddWorkspaceGroupMemberResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddWorkspaceGroupMemberResponseModelBuilder {
    status: Option<String>,
}

impl AddWorkspaceGroupMemberResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddWorkspaceGroupMemberResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](AddWorkspaceGroupMemberResponseModelBuilder::status)
    pub fn build(self) -> Result<AddWorkspaceGroupMemberResponseModel, BuildError> {
        Ok(AddWorkspaceGroupMemberResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
