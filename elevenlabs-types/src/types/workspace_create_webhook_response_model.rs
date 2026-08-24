pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceCreateWebhookResponseModel {
    #[serde(default)]
    pub webhook_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
}

impl WorkspaceCreateWebhookResponseModel {
    pub fn builder() -> WorkspaceCreateWebhookResponseModelBuilder {
        <WorkspaceCreateWebhookResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceCreateWebhookResponseModelBuilder {
    webhook_id: Option<String>,
    webhook_secret: Option<String>,
}

impl WorkspaceCreateWebhookResponseModelBuilder {
    pub fn webhook_id(mut self, value: impl Into<String>) -> Self {
        self.webhook_id = Some(value.into());
        self
    }

    pub fn webhook_secret(mut self, value: impl Into<String>) -> Self {
        self.webhook_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceCreateWebhookResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhook_id`](WorkspaceCreateWebhookResponseModelBuilder::webhook_id)
    pub fn build(self) -> Result<WorkspaceCreateWebhookResponseModel, BuildError> {
        Ok(WorkspaceCreateWebhookResponseModel {
            webhook_id: self.webhook_id.ok_or_else(|| BuildError::missing_field("webhook_id"))?,
            webhook_secret: self.webhook_secret,
        })
    }
}
