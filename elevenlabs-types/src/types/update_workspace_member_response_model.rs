pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWorkspaceMemberResponseModel {
    /// The status of the workspace member update request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl UpdateWorkspaceMemberResponseModel {
    pub fn builder() -> UpdateWorkspaceMemberResponseModelBuilder {
        <UpdateWorkspaceMemberResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWorkspaceMemberResponseModelBuilder {
    status: Option<String>,
}

impl UpdateWorkspaceMemberResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWorkspaceMemberResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](UpdateWorkspaceMemberResponseModelBuilder::status)
    pub fn build(self) -> Result<UpdateWorkspaceMemberResponseModel, BuildError> {
        Ok(UpdateWorkspaceMemberResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
