pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceWebhookListResponseModel {
    /// List of webhooks currently configured for the workspace
    #[serde(default)]
    pub webhooks: Vec<WorkspaceWebhookResponseModel>,
}

impl WorkspaceWebhookListResponseModel {
    pub fn builder() -> WorkspaceWebhookListResponseModelBuilder {
        <WorkspaceWebhookListResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceWebhookListResponseModelBuilder {
    webhooks: Option<Vec<WorkspaceWebhookResponseModel>>,
}

impl WorkspaceWebhookListResponseModelBuilder {
    pub fn webhooks(mut self, value: Vec<WorkspaceWebhookResponseModel>) -> Self {
        self.webhooks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceWebhookListResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhooks`](WorkspaceWebhookListResponseModelBuilder::webhooks)
    pub fn build(self) -> Result<WorkspaceWebhookListResponseModel, BuildError> {
        Ok(WorkspaceWebhookListResponseModel {
            webhooks: self.webhooks.ok_or_else(|| BuildError::missing_field("webhooks"))?,
        })
    }
}
