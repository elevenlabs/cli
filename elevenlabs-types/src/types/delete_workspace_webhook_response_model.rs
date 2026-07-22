pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteWorkspaceWebhookResponseModel {
    /// The status of the workspace webhook deletion request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl DeleteWorkspaceWebhookResponseModel {
    pub fn builder() -> DeleteWorkspaceWebhookResponseModelBuilder {
        <DeleteWorkspaceWebhookResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteWorkspaceWebhookResponseModelBuilder {
    status: Option<String>,
}

impl DeleteWorkspaceWebhookResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteWorkspaceWebhookResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DeleteWorkspaceWebhookResponseModelBuilder::status)
    pub fn build(self) -> Result<DeleteWorkspaceWebhookResponseModel, BuildError> {
        Ok(DeleteWorkspaceWebhookResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
