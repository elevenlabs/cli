pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchWorkspaceWebhookResponseModel {
    /// The status of the workspace webhook patch request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl PatchWorkspaceWebhookResponseModel {
    pub fn builder() -> PatchWorkspaceWebhookResponseModelBuilder {
        <PatchWorkspaceWebhookResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchWorkspaceWebhookResponseModelBuilder {
    status: Option<String>,
}

impl PatchWorkspaceWebhookResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PatchWorkspaceWebhookResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](PatchWorkspaceWebhookResponseModelBuilder::status)
    pub fn build(self) -> Result<PatchWorkspaceWebhookResponseModel, BuildError> {
        Ok(PatchWorkspaceWebhookResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
